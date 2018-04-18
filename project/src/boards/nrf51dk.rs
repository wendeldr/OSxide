use nrf51;
use cortex_m;
use cortex_m::interrupt::{Mutex};
use cortex_m_semihosting::hio::{HStdout};

use core::cell::RefCell;

//mod peripherals;
use boards::board::Board;
use boards::peripherals::leds::Led;
use boards::peripherals::buttons::Button;
use boards::peripherals::timers::Timer;


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
    //Leds: [Led_S; 4],
}


impl Nrf51dk {
    // What to do when the board comes up
    pub fn new() -> Nrf51dk {
        Nrf51dk { }
    }


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
            *PERIPH.borrow(cs).borrow_mut() = Some(p);

        });
    
        // timer0 with a frequency of 1000000
        let timer0 = Timer::new(0, 1000000);

        //Lets use timer0 as a systick
        //let delay: u32 = 5 * 1000000; // five second delay
        let delay: u32 = 1000;
        timer0.init(delay);
        timer0.start();

        Led::init();
        Button::init();

    }




}

impl Board for Nrf51dk {
    fn new( ) -> Nrf51dk {
        Nrf51dk {}
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
