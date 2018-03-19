

//! # Chip interface for the NORDIC NRF51XX series 
//!
//! This module is responsible for handeling the nrf51 series chip
extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate nrf51;
extern crate nrf51_hal as hal;

use chips::capsules::timer::{self, Timer};
use chips::chip::Chip;

use self::cortex_m::interrupt::{self, Mutex};
use self::cortex_m_semihosting::hio;
use core::fmt::Write;

pub struct NRF51 {}

impl NRF51 {
    
    /// Initializes the peripherals on the chip
    pub fn init(&self) {
        self.write("Initializing Devices");

        if let Some(p) = nrf51::Peripherals::take() {
            //let c: TIMER0 = unsafe{ mem::transmute(p.TIMER2) };
            //let mut tim0 = Timer::new(c);
            p.TIMER2.tasks_stop.write(|w| unsafe { w.bits(1) });
        }
    }

    /// Writes to stdout via gdb
    ///
    /// writes "message" to stdout
    pub fn write(&self, message: &'static str) {
        let mut stdout = hio::hstdout().unwrap();
        writeln!(stdout, "{}", message).unwrap()
    }
}

impl Chip for NRF51 {
    fn new() -> NRF51 {
        // Retturns an instance of NRF51
        NRF51 { }
    }
}
