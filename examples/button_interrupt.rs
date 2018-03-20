#![feature(used)]
#![feature(const_fn)]
#![no_std]
#[macro_use( interrupt)]

#[macro_use]

extern crate nrf51;
extern crate nrf51_hal;
extern crate cortex_m;
extern crate cortex_m_semihosting;




use cortex_m::interrupt::{self, Mutex};
use cortex_m_semihosting::hio::{self, HStdout};
use nrf51::{Interrupt, GPIOTE};
use nrf51_hal::gpio::GpioExt;

use cortex_m::peripheral::Peripherals;
use core::cell::RefCell;
use core::fmt::Write;
use core::ops::DerefMut;

static HSTDOUT: Mutex<RefCell<Option<HStdout>>> = Mutex::new(RefCell::new(None));
static GPIO: Mutex<RefCell<Option<GPIOTE>>> = Mutex::new(RefCell::new(None));

fn main() {



    let p = nrf51::Peripherals::take().unwrap();
    let mut cp = cortex_m::Peripherals::take().unwrap();

    cortex_m::interrupt::free(move |cs| {
        let hstdout = HSTDOUT.borrow(cs);
        if let Ok(fd) = hio::hstdout() {
            *hstdout.borrow_mut() = Some(fd);
        }

        if let Some(hstdout) = hstdout.borrow_mut().as_mut() {
            writeln!(*hstdout, "Hello").ok();
        }

        /* Enable external GPIO interrupts */
        cp.NVIC.enable(nrf51::Interrupt::GPIOTE);
        cp.NVIC.clear_pending(nrf51::Interrupt::GPIOTE);
        
        let gpio = p.GPIO.split();


        let _ = gpio.pin17.into_pull_up_input();
        let _ = gpio.pin18.into_pull_up_input();
        let _ = gpio.pin19.into_pull_up_input();
        let _ = gpio.pin20.into_pull_up_input();
        
        *GPIO.borrow(cs).borrow_mut() = Some(p.GPIOTE);
        //PERIPH.borrow(cs).borrow_mut() = Some(p);

        /* Split GPIO pins */

        /* Configure button GPIOs as inputs */
        //let _ = gpio.pin26.into_floating_input();
        //let _ = gpio.pin17.into_floating_input();

        //let gpio = p.GPIO.split();
        Buttons::setup();

    });
}


pub struct Buttons {

}

impl Buttons {

    fn setup() {

        //Interrupt free zone!
        cortex_m::interrupt::free(|cs| {
            //cs can only be called in here and ensures that
            // we can't be interrupted
            if let Some(gpiote) = GPIO.borrow(cs).borrow().as_ref() {
                //if we were sucessfull we should have just borrowed p.gpiote

                /* Set up GPIO 17 (button 1) to generate an interrupt when pulled down */
                gpiote.config[0].write(|w| unsafe { w.mode().event().psel().bits(17).polarity().lo_to_hi()});
                gpiote.intenset.write(|w| w.in0().set_bit());
                gpiote.events_in[0].write(|w| unsafe { w.bits(0) });

                /* Set up GPIO 18 (button 2) to generate an interrupt when pulled down */
                gpiote.config[1].write(|w| unsafe { w.mode().event().psel().bits(18).polarity().lo_to_hi()});
                gpiote.intenset.write(|w| w.in1().set_bit());
                gpiote.events_in[1].write(|w| unsafe { w.bits(0) });

                /* Set up GPIO 19 (button 3) to generate an interrupt when pulled down */
                gpiote.config[2].write(|w| unsafe { w.mode().event().psel().bits(19).polarity().lo_to_hi()});
                gpiote.intenset.write(|w| w.in2().set_bit());
                gpiote.events_in[2].write(|w| unsafe { w.bits(0) });

                /* Set up GPIO 18 (button 4) to generate an interrupt when pulled down */
                gpiote.config[3].write(|w| unsafe { w.mode().event().psel().bits(20).polarity().lo_to_hi()});
                gpiote.intenset.write(|w| w.in3().set_bit());
                gpiote.events_in[3].write(|w| unsafe { w.bits(0) });

            }
        });
    }
}


/* Define an exception, i.e. function to call when exception occurs. Here if we receive an
 * interrupt from a button press, the printbuttons function will be called */
interrupt!(GPIOTE, printbuttons);

fn printbuttons() {
    /* Enter critical section */
    cortex_m::interrupt::free(|cs| {
        if let Some(gpiote) = GPIO.borrow(cs).borrow().as_ref() {
            let button1 = gpiote.events_in[0].read().bits() != 0;
            let button2 = gpiote.events_in[1].read().bits() != 0;
            let button3 = gpiote.events_in[2].read().bits() != 0;
            let button4 = gpiote.events_in[3].read().bits() != 0;

            /* Print buttons to the serial console */
            let hstdout = HSTDOUT.borrow(cs);
            if let Ok(fd) = hio::hstdout() {
                *hstdout.borrow_mut() = Some(fd);
            }

            if let Some(hstdout) = hstdout.borrow_mut().as_mut() {

                writeln!(*hstdout, "button press {}", button1).ok();
                writeln!(*hstdout, "button press {}", button2).ok();
                writeln!(*hstdout, "button press {}", button3).ok();
                writeln!(*hstdout, "button press {}", button4).ok();
                writeln!(*hstdout, "\n").ok();
            }

            /* Clear events */
            gpiote.events_in[0].write(|w| unsafe { w.bits(0) });
            gpiote.events_in[1].write(|w| unsafe { w.bits(0) });
            gpiote.events_in[2].write(|w| unsafe { w.bits(0) });
            gpiote.events_in[3].write(|w| unsafe { w.bits(0) });
        }
    });
}