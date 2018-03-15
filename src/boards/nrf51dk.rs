
#[allow(non_camel_case_types)]
#[macro_use(exception)]
#[macro_use(interrupt)]


use boards::board::Board;
//use boards::interupts;

/// (DOCS cotex_m)[https://docs.rs/cortex-m/0.4.3/cortex_m/]
pub extern crate cortex_m;
pub extern crate cortex_m_rt;
pub extern crate cortex_m_semihosting;

// Provides HAL
extern crate nrf51_hal as hal;
extern crate nrf51;

// Provides hardware
pub use self::nrf51::*;
pub use self::nrf51::{Peripherals};

use self::hal::gpio::GpioExt;

use self::cortex_m::interrupt::{self, Mutex};
use self::cortex_m::asm;
use self::cortex_m_semihosting::hio::{self, HStdout};



use core::cell::RefCell;
use core::fmt::Write;

//static GPIO: Mutex<RefCell<Option<nrf51::GPIOTE>>> = Mutex::new(RefCell::new(None));
//static NVIC: Mutex<RefCell<Option<cortex_m::peripheral::NVIC>>> = Mutex::new(RefCell::new(None));

pub struct nrf51dk { name: &'static str }


impl nrf51dk {

    // What to do when the board comes up
    pub fn init(&self) {
        let mut stdout = hio::hstdout().unwrap();
        writeln!(stdout, "Initializing devices").unwrap();

        nrf51dk::enable_interrupts();

        nrf51dk::setup_buttons();

        nrf51dk::set_clock(); 

        //Initialize clock

        //set interupts

        //enable interrupts

    }

    fn enable_interrupts() {
        if let Some(global_p) = cortex_m::peripheral::Peripherals::take() {
            interrupt::free( |cs| {
                let mut nvic = global_p.NVIC;
                //ADD INTERUPTS HERE
                nvic.enable(Interrupt::TIMER2);
                *NVIC.borrow(cs).borrow_mut() = Some(nvic);
            });
        }

    }

    fn set_clock() {
        if let Some(global_p) = cortex_m::peripheral::Peripherals::take() {
            interrupt::free( |cs| {
                // systick does not exist
            });
        }
    }

    fn setup_buttons() {
        if let Some(p) = Peripherals::take() {
              /* Split GPIO pins */
            let gpio = p.GPIO.split();

            /* Configure button GPIOs as inputs */
            
            /*the internal resistor needs to be set to pull up*/
            gpio.pin17.into_pull_up_input();
            gpio.pin18.into_pull_up_input();
            gpio.pin19.into_pull_up_input();
            gpio.pin20.into_pull_up_input();

        }
    }


    pub fn service_pending_interrupt() {

    }
}

impl Board for nrf51dk {
    fn new(name: &'static str) -> nrf51dk {
        nrf51dk { name: name}
    }
}

