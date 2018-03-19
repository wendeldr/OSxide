#![deny(warnings)]
#![feature(const_fn)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_semihosting;
#[macro_use( interrupt)]
extern crate nrf51;
extern crate nrf51_hal;

use core::cell::RefCell;
use core::fmt::Write;

use cortex_m::interrupt::{self, Mutex};
use cortex_m_semihosting::hio::{self, HStdout};
use nrf51::{Interrupt};

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

 
        set_timer();

        let mut nvic = global_p.NVIC;
        nvic.enable(Interrupt::TIMER0);
        *NVIC.borrow(cs).borrow_mut() = Some(nvic);

    });
}


fn set_timer() {

    let p = nrf51::Peripherals::take().unwrap();


    let us: u32 = 2000000; // 2 secondsish?

    // Set timer to 32 bit and prescalar to 4
    p.TIMER0.bitmode.write(|w| w.bitmode()._32bit());
    p.TIMER0.prescaler.write(|w| unsafe { w.prescaler().bits(4) });
    
    //Set the Timer intent
    p.TIMER0.intenset.write(|w| w.compare0().set());
    p.TIMER0.shorts.write(|w| w.compare0_clear().enabled());
    p.TIMER0.events_compare[0].write(|w| unsafe { w.bits(0) });

    /* Program counter compare register with value */
    p.TIMER0.cc[0].write(|w| unsafe { w.bits(us) });

    /* Clear current counter value */
    p.TIMER0.tasks_clear.write(|w| unsafe { w.bits(1) });

    /* Start counting */
    p.TIMER0.tasks_start.write(|w| unsafe { w.bits(1) });

}

// sets interrupt
interrupt!(TIMER0, timer0_irqhandler);


fn timer0_irqhandler() {
    interrupt::free(|cs| {
        let hstdout = HSTDOUT.borrow(cs);
        if let Some(hstdout) = hstdout.borrow_mut().as_mut() {
            writeln!(*hstdout, "Tock0").ok();
        }
    });


    // fires but does not clear for some reason
    //let global_p = cortex_m::Peripherals::take().unwrap();
    let p = nrf51::Peripherals::take().unwrap();

    /* Clear current counter value */
    p.TIMER0.tasks_clear.write(|w| unsafe { w.bits(1) });
    p.TIMER0.intenclr.write(|w| w.compare0().clear());
    p.TIMER0.intenset.write(|w| w.compare0().set());

    set_timer();
}