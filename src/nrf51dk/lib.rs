#![no_std]
 #![feature(use_extern_macros)]
///
/// # Provides Board support
/// 
/// To see what these provide checkout the following projects
/// 
/// [nrf51-hal](https://docs.rs/nrf51-hal/0.4.2/nrf51_hal/)
/// Provides:
///     * delay
///     * gpio
///     * i2c
///     * prelude
///     * rng (random number generator)
///     * timer
///
/// [nrf51](https://docs.rs/nrf51/0.4.0/nrf51/index.html)
/// Provides:
///     * aar	Accelerated Address Resolver.
///     * adc	Analog to digital converter.
///     * ccm	AES CCM Mode Encryption.
///     * clock	Clock control.
///     * ecb	AES ECB Mode Encryption.
///     * ficr	Factory Information Configuration.
///     * gpio	General purpose input and output.
///     * gpiote	GPIO tasks and events.
///     * lpcomp	Low power comparator.
///     * nvmc	Non Volatile Memory Controller.
///     * power	Power Control.
///     * ppi	PPI controller.
///     * qdec	Rotary decoder.
///     * radio	The radio.
///     * rng	Random Number Generator.
///     * rtc0	Real time counter 0.
///     * spi0	SPI master 0.
///     * spim1	SPI master with easyDMA 1.
///     * spis1	SPI slave 1.
///     * swi	SW Interrupts.
///     * temp	Temperature Sensor.
///     * timer0	Timer 0.
///     * twi0	Two-wire interface master 0.
///     * uart0	Universal Asynchronous Receiver/Transmitter.
///     * uicr	User Information Configuration.
///     * wdt	Watchdog Timer.


pub extern crate cortex_m;
pub extern crate cortex_m_rt;
pub extern crate cortex_m_semihosting;

// Provides HAL
pub extern crate nrf51_hal as hal;

extern crate nrf51;

// Provides hardware
pub use nrf51::*;
pub use nrf51::Interrupt::*;

use core::cell::RefCell;

use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::Peripheral;
use cortex_m_semihosting::hio;
use core::fmt::Write;



static GPIO: Mutex<RefCell<Option<nrf51::GPIOTE>>> = Mutex::new(RefCell::new(None));



pub struct nrf51dk_local { name: &'static str }

pub trait Board {
     fn new(name: &'static str) -> Self;
}



impl nrf51dk_local {

    // What to do when the board comes up
    pub fn init(&self) {
        let mut stdout = hio::hstdout().unwrap();
        writeln!(stdout, "Initializing devices").unwrap();


        // initialize interrupts
        if let Some(p) = hal::peripherals::Take() {
            

        }
    }

    pub fn add_task() {
        let mut stdout = hio::hstdout().unwrap();
        writeln!(stdout, "Adding Task").unwrap();
    }
}

impl Board for nrf51dk_local {
    fn new(name: &'static str) -> nrf51dk_local {
        nrf51dk_local { name: name}
    }
}


#[no_mangle]
#[allow(non_snake_case)]
interrupt!(GPIOTE, GPIOTE_Handler);
fn GPIOTE_Handler() {
    
}