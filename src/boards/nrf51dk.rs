
#[allow(non_camel_case_types)]

use boards::board::Board;

pub extern crate cortex_m;
pub extern crate cortex_m_rt;
pub extern crate cortex_m_semihosting;

// Provides HAL
pub extern crate nrf51_hal as hal;

extern crate nrf51;

// Provides hardware
pub use self::nrf51::*;
pub use self::nrf51::Interrupt::*;

use self::hal::gpio::GpioExt;

use self::cortex_m::interrupt::Mutex;
use self::cortex_m::peripheral::Peripheral;
use self::cortex_m_semihosting::hio;

use core::cell::RefCell;
use core::fmt::Write;


static GPIO: Mutex<RefCell<Option<nrf51::GPIOTE>>> = Mutex::new(RefCell::new(None));


pub struct nrf51dk { name: &'static str }


impl nrf51dk {

    // What to do when the board comes up
    pub fn init(&self) {
        let mut stdout = hio::hstdout().unwrap();
        writeln!(stdout, "Initializing devices").unwrap();

        nrf51dk::setup_buttons()

        //Initialize clock

        //set interupts

        //enable interrupts

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

    pub fn add_task() {
        let mut stdout = hio::hstdout().unwrap();
        writeln!(stdout, "Adding Task").unwrap();
    }

    pub fn service_pending_interrupt() {
        
    }
}

impl Board for nrf51dk {
    fn new(name: &'static str) -> nrf51dk {
        nrf51dk { name: name}
    }
}

