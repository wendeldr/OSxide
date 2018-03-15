#![feature(used)]
#![feature(const_fn)]
#![no_std]

#[macro_use]
extern crate nrf51_hal;

use nrf51_hal::cortex_m;
use nrf51_hal::hal::prelude::*;
use nrf51_hal::hal::serial;
use nrf51_hal::hal::serial::BAUD115200;

use cortex_m::peripheral::Peripherals;
use cortex_m::interrupt::Mutex;

use core::cell::RefCell;
use core::fmt::Write;
use core::ops::DerefMut;

static GPIO: Mutex<RefCell<Option<nrf51_hal::GPIOTE>>> = Mutex::new(RefCell::new(None));
static TX: Mutex<RefCell<Option<serial::Tx<nrf51_hal::UART0>>>> = Mutex::new(RefCell::new(None));

fn main() {
    if let (Some(p), Some(mut cp)) = (nrf51_hal::Peripherals::take(), Peripherals::take()) {
        cortex_m::interrupt::free(move |cs| {
            /* Enable external GPIO interrupts */
            cp.NVIC.enable(nrf51_hal::Interrupt::GPIOTE);
            cp.NVIC.clear_pending(nrf51_hal::Interrupt::GPIOTE);

            /* Split GPIO pins */
            let gpio = p.GPIO.split();

            /* Configure button GPIOs as inputs */
            let _ = gpio.pin26.into_floating_input();
            let _ = gpio.pin17.into_floating_input();

            /* Set up GPIO 17 (button A) to generate an interrupt when pulled down */
            p.GPIOTE.config[0]
                .write(|w| unsafe { w.mode().event().psel().bits(17).polarity().hi_to_lo() });
            p.GPIOTE.intenset.write(|w| w.in0().set_bit());
            p.GPIOTE.events_in[0].write(|w| unsafe { w.bits(0) });

            /* Set up GPIO 26 (button B) to generate an interrupt when pulled down */
            p.GPIOTE.config[1]
                .write(|w| unsafe { w.mode().event().psel().bits(26).polarity().hi_to_lo() });
            p.GPIOTE.intenset.write(|w| w.in1().set_bit());
            p.GPIOTE.events_in[1].write(|w| unsafe { w.bits(0) });

            *GPIO.borrow(cs).borrow_mut() = Some(p.GPIOTE);

            /* Configure RX and TX pins accordingly */
            let tx = gpio.pin24.into_push_pull_output().downgrade();
            let rx = gpio.pin25.into_floating_input().downgrade();

            /* Set up serial port using the prepared pins */
            let (mut tx, _) = serial::Serial::uart0(p.UART0, tx, rx, BAUD115200).split();

            let _ = write!(
                TxBuffer(&mut tx),
                "\n\rWelcome to the buttons demo. Press buttons A and/or B for some action.\n\r",
            );
            *TX.borrow(cs).borrow_mut() = Some(tx);
        });
    }
}

/* Define an exception, i.e. function to call when exception occurs. Here if we receive an
 * interrupt from a button press, the printbuttons function will be called */
interrupt!(GPIOTE, printbuttons);

fn printbuttons() {
    /* Enter critical section */
    cortex_m::interrupt::free(|cs| {
        if let (Some(gpiote), &mut Some(ref mut tx)) = (
            GPIO.borrow(cs).borrow().as_ref(),
            TX.borrow(cs).borrow_mut().deref_mut(),
        ) {
            let buttonapressed = gpiote.events_in[0].read().bits() != 0;
            let buttonbpressed = gpiote.events_in[1].read().bits() != 0;

            /* Print buttons to the serial console */
            let _ = write!(
                TxBuffer(tx),
                "Button pressed {}\n\r",
                match (buttonapressed, buttonbpressed) {
                    (true, false) => "A",
                    (false, true) => "B",
                    (true, true) => "A + B",
                    _ => "A + B",
                }
            );

            /* Clear events */
            gpiote.events_in[0].write(|w| unsafe { w.bits(0) });
            gpiote.events_in[1].write(|w| unsafe { w.bits(0) });
        }
    });
}

struct TxBuffer<'a>(&'a mut serial::Tx<nrf51_hal::UART0>);

impl<'a> core::fmt::Write for TxBuffer<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let _ = s.as_bytes()
            .into_iter()
            .map(|c| block!(self.0.write(*c)))
            .last();
        Ok(())
    }
}