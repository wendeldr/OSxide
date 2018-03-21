
extern crate nrf51;
extern crate nrf51_hal;
extern crate cortex_m;
extern crate cortex_m_semihosting;


use boards::board::Board;
use cortex_m_semihosting::hio::{HStdout};
use nrf51::{GPIOTE};
use cortex_m::interrupt::{Mutex};
//use nrf51_hal::gpio::GpioExt::{self};


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

interrupt!(GPIOTE, button_irq_handler);
fn button_irq_handler() {
    /* Enter critical section */
    cortex_m::interrupt::free(|cs| {
        if let Some(p) = PERIPH.borrow(cs).borrow().as_ref() {

            let button1 = p.GPIOTE.events_in[0].read().bits() != 0;
            let button2 = p.GPIOTE.events_in[1].read().bits() != 0;
            let button3 = p.GPIOTE.events_in[2].read().bits() != 0;
            let button4 = p.GPIOTE.events_in[3].read().bits() != 0;



            let hstdout = HSTDOUT.borrow(cs);

            if let Some(hstdout) = hstdout.borrow_mut().as_mut() {
                writeln!(*hstdout, "button1 {} - button2 {} - button3 {} - button4 {}", button1, button2, button3, button4).ok();
            }

            //clear button 1 events
            p.GPIOTE.events_in[0].write(|w| unsafe { w.bits(0) });
            p.GPIOTE.events_in[1].write(|w| unsafe { w.bits(0) });
            p.GPIOTE.events_in[2].write(|w| unsafe { w.bits(0) });
            p.GPIOTE.events_in[3].write(|w| unsafe { w.bits(0) });

        }
    });
}