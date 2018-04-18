
//! ### Pin configuration
//! * 0 -> LED1 (pin 21)
//! * 1 -> LED2 (pin 22)
//! * 2 -> LED3 (pin 23)
//! * 3 -> LED4 (pin 24)
//! * 5 -> BUTTON1 (pin 17)
//! * 6 -> BUTTON2 (pin 18)
//! * 7 -> BUTTON3 (pin 19)
//! * 8 -> BUTTON4 (pin 20)
//! * 9 -> P0.01   (bottom left header)
//! * 10 -> P0.02   (bottom left header)
//! * 11 -> P0.03   (bottom left header)
//! * 12 -> P0.04   (bottom left header)
//! * 12 -> P0.05   (bottom left header)
//! * 13 -> P0.06   (bottom left header)
//! * 14 -> P0.19   (mid right header)
//! * 15 -> P0.18   (mid right header)
//! * 16 -> P0.17   (mid right header)
//! * 17 -> P0.16   (mid right header)
//! * 18 -> P0.15   (mid right header)
//! * 19 -> P0.14   (mid right header)
//! * 20 -> P0.13   (mid right header)
//! *extern crate cortex_m;

#![feature(used)]
#![no_std]

extern crate nrf51;
extern crate cortex_m_semihosting;
extern crate nrf51_hal as hal;

use cortex_m_semihosting::hio::{self, HStdout};
use hal::prelude::*;

use core::fmt::Write;

fn main() {
     let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Hello, world!").unwrap();


    if let Some(p) = nrf51::Peripherals::take() {
        /* Split GPIO pins */
        let gpio = p.GPIO.split();

        /* Set 2 columns to output to control LED states */
        let mut led_1 = gpio.pin21.into_push_pull_output();
        let mut led_2 = gpio.pin22.into_push_pull_output();
        let mut led_3 = gpio.pin23.into_push_pull_output();
        let mut led_4 = gpio.pin24.into_push_pull_output();

        /* Configure button GPIOs as inputs */
        /*the internal resistor needs to be set to pull up*/
        let button_1 = gpio.pin17.into_pull_up_input();
        let button_2 = gpio.pin18.into_pull_up_input();
        let button_3 = gpio.pin19.into_pull_up_input();
        let button_4 = gpio.pin20.into_pull_up_input();

        loop {
            if button_1.is_high() {
                led_1.set_high();
            } else {
                led_1.set_low();
            }

            if button_2.is_high() {
                led_2.set_high();
            } else {
                led_2.set_low();
            }

            if button_3.is_high() {
                led_3.set_high();
            } else {
                led_3.set_low();
            }

            if button_4.is_high() {
                led_4.set_high();
            } else {
                led_4.set_low();
            }
        }


    }
}
