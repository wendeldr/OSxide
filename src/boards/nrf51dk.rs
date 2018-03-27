
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
use boards::peripherals::timers::Timer;


use boards::interrupt::{Interrupt};


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
    pub interrupt: Interrupt
}


impl Nrf51dk {
    // What to do when the board comes up
    pub fn init(&self) {
        cortex_m::interrupt::free(|cs| {

            /* Initilize the interrupts on cpu*/
            // TODO should the device structs handle these?
            let mut cp = cortex_m::Peripherals::take().unwrap();
            cp.NVIC.enable(nrf51::Interrupt::GPIOTE);
            cp.NVIC.clear_pending(nrf51::Interrupt::GPIOTE);
            cp.NVIC.enable(nrf51::Interrupt::TIMER0);
            cp.NVIC.clear_pending(nrf51::Interrupt::TIMER0);

            

            let p = nrf51::Peripherals::take().unwrap(); // todo don't unwrap
            // lets borrow Peripherals
            let timer = Timer::new(0);

            let delay: u32 = 5 * 1000000; // five second delay
            timer.init(delay, 1000000);
            timer.resume();

            //timer.init(1000000);

            *PERIPH.borrow(cs).borrow_mut() = Some(p);

        });

        Led::init();
        Button::init();

        let delay: u32 = 2 * 1000000; // five second delay

        

        //Timer::init(delay);
    

        //TODO not sure if this is necessary
        //Nrf51dk{ interrupt: Interrupt::new() }

    }
}

impl Board for Nrf51dk {
    fn new( ) -> Nrf51dk {
        Nrf51dk { interrupt: Interrupt::new() }
    }

    fn led_on(&self, i: usize) {
        LEDS[i].on();
    }

    fn led_off(&self, i: usize) {
        LEDS[i].off();
    }
    
    fn led_toggle(&self, i: usize) {
        LEDS[i].toggle();
    }
}
