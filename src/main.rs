//! Prints "Hello, world!" on the OpenOCD console using semihosting
//!
//! ---

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
//! * 21 -> P0.12   (mid right header)

#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;

//device specific svd
// nrf51 provides interrupts
// well isn't that nifty
// hal provides access to gpio pins
extern crate nrf51_hal as hal;

use core::fmt::Write;

use cortex_m::asm;
use cortex_m_semihosting::hio;

extern crate nrf51;

pub use nrf51::*;
pub use nrf51::interrupt::*;

use hal::gpio::gpio::Parts;

fn init() {

}


fn main() {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Hello, world!").unwrap();


    let button_a = gpio::pin17.into_floating_input();

    loop {
        if button_a.is_high() {
                writeln!(stdout, "BUTTON!").unwrap();
        }
    }
}
