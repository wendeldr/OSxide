
extern crate nrf51;
extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate cortex_m_rt;


use boards::board::Board;
use cortex_m_semihosting::hio::{HStdout};

use cortex_m::interrupt::{Mutex};

use core::cell::RefCell;
use core::fmt::Write;

//mod peripherals;
use boards::peripherals::leds::Led;
use boards::peripherals::buttons::Button;


pub static HSTDOUT: Mutex<RefCell<Option<HStdout>>> = Mutex::new(RefCell::new(None));
pub static PERIPH: Mutex<RefCell<Option<nrf51::Peripherals>>> = Mutex::new(RefCell::new(None));


/* 
    TODO Genericise these so that they get passed 
    to the boards instead of being a global static
    variable
*/

pub static LEDS: [Led; 4] = [
    Led { i: 21 }, 
    Led { i: 22 }, 
    Led { i: 23 }, 
    Led { i: 24 }
    ];

pub static BUTTONS: [Button; 4] = [
    Button { i: 17 }, 
    Button { i: 18 }, 
    Button { i: 19 }, 
    Button { i: 20 }
    ];
 

pub struct Nrf51dk {

}


impl Nrf51dk {
    // What to do when the board comes up
    pub fn init(&self) -> Self {
        cortex_m::interrupt::free(|cs| {

            let hstdout = HSTDOUT.borrow(cs);

            if let Some(hstdout) = hstdout.borrow_mut().as_mut() {
                writeln!(*hstdout, "NRF51Dk Initialization").ok();
            }

            /* Initilize the interrupts on cpu*/
            // TODO should the device structs handle these?
            let mut cp = cortex_m::Peripherals::take().unwrap();
            cp.NVIC.enable(nrf51::Interrupt::GPIOTE);
            cp.NVIC.clear_pending(nrf51::Interrupt::GPIOTE);
            

            let p = nrf51::Peripherals::take().unwrap(); // todo don't unwrap
            // lets borrow Peripherals
            *PERIPH.borrow(cs).borrow_mut() = Some(p);

        });

        Led::init();
        Button::init();

        //TODO not sure if this is necessary
        Nrf51dk{ }

    }
}

impl Board for Nrf51dk {
    fn new( ) -> Nrf51dk {
        Nrf51dk { }
    }

    fn led_on(&self, i: usize) {
        LEDS[i].on();
    }

    fn led_off(&self, i: usize) {
        LEDS[i].off();
    }
}
