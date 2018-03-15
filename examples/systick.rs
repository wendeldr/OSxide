#![deny(warnings)]
#![feature(const_fn)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_semihosting;
#[macro_use(exception, interrupt)]
extern crate nrf51;

use core::cell::RefCell;
use core::fmt::Write;

use cortex_m::interrupt::{self, Mutex};
use cortex_m_semihosting::hio::{self, HStdout};
use nrf51::Interrupt;

static HSTDOUT: Mutex<RefCell<Option<HStdout>>> = Mutex::new(RefCell::new(None));
static NVIC: Mutex<RefCell<Option<cortex_m::peripheral::NVIC>>> = Mutex::new(RefCell::new(None));

fn main() {
    let global_p = cortex_m::Peripherals::take().unwrap();
    interrupt::free(|cs| {
        let hstdout = HSTDOUT.borrow(cs);
        if let Ok(fd) = hio::hstdout() {
            *hstdout.borrow_mut() = Some(fd);
        }

        if let Some(hstdout) = hstdout.borrow_mut().as_mut() {
            writeln!(*hstdout, "Hello").ok();
        }

        let mut nvic = global_p.NVIC;
        nvic.enable(Interrupt::TIMER0);
        *NVIC.borrow(cs).borrow_mut() = Some(nvic);

        tick();

    });
}

fn tick() {
    interrupt::free(|cs| {
        let hstdout = HSTDOUT.borrow(cs);
        if let Some(hstdout) = hstdout.borrow_mut().as_mut() {
            writeln!(*hstdout, "Tick").ok();
        }

        if let Some(nvic) = NVIC.borrow(cs).borrow_mut().as_mut() {
            nvic.set_pending(Interrupt::TIMER0);
        }
    });
}

exception!(HARD_FAULT, Break::hard_fault_handler);

struct Break {
}

impl Break {
    fn hard_fault_handler() {
        interrupt::free(|cs| {
            let hstdout = HSTDOUT.borrow(cs);
            if let Some(hstdout) = hstdout.borrow_mut().as_mut() {
                writeln!(*hstdout, "Hard Fault!").ok();
            }
        });
    }

}


interrupt!(TIMER0, Timer::timer0_irqhandler);
interrupt!(TIMER1, Timer::timer1_irqhandler);
interrupt!(TIMER2, Timer::timer2_irqhandler);


struct Timer {
}

impl Timer {

    fn timer0_irqhandler() {
        interrupt::free(|cs| {
            let hstdout = HSTDOUT.borrow(cs);
            if let Some(hstdout) = hstdout.borrow_mut().as_mut() {
                writeln!(*hstdout, "Tock0").ok();
            }
        });
    }

    fn timer1_irqhandler() {
         interrupt::free(|cs| {
            let hstdout = HSTDOUT.borrow(cs);
            if let Some(hstdout) = hstdout.borrow_mut().as_mut() {
                writeln!(*hstdout, "Tock1").ok();
            }
        });
    }

    fn timer2_irqhandler() {
         interrupt::free(|cs| {
            let hstdout = HSTDOUT.borrow(cs);
            if let Some(hstdout) = hstdout.borrow_mut().as_mut() {
                writeln!(*hstdout, "Tock2").ok();
            }
        });
    }
}