#![feature(used)]
#![feature(const_fn)]
#![no_std]
#[macro_use( interrupt)]

#[macro_use]

extern crate nrf51;
extern crate nrf51_hal;
extern crate cortex_m;
extern crate cortex_m_semihosting;


use cortex_m::interrupt::{Mutex};
use cortex_m_semihosting::hio::{self, HStdout};
use nrf51::{GPIOTE};
use nrf51_hal::gpio::GpioExt::{self};

use core::cell::RefCell;
use core::fmt::Write;

static HSTDOUT: Mutex<RefCell<Option<HStdout>>> = Mutex::new(RefCell::new(None));
static PERIPH: Mutex<RefCell<Option<nrf51::Peripherals>>> = Mutex::new(RefCell::new(None));

pub static LEDS: [Led; 4] = [Led { i: 21 }, 
                             Led { i: 22 }, 
                             Led { i: 23 }, 
                             Led { i: 24 }];

pub static BUTTONS: [Button; 4] = [Button { i: 17 }, Button { i: 18 }, Button { i: 19 }, Button { i: 20 }];


fn main() {

    cortex_m::interrupt::free(move |cs| {

        // lets set up std out!
        let hstdout = HSTDOUT.borrow(cs);

        if let Ok(fd) = hio::hstdout() {
            *hstdout.borrow_mut() = Some(fd);
        }

        if let Some(hstdout) = hstdout.borrow_mut().as_mut() {
            writeln!(*hstdout, "Main!").ok();
        }
    });

    // initialize the board
    let _board = Board::new();

    loop {
        // do nothing..
        // we're kinda just waiting to be told what to do here..
    }
}

pub struct Board {
    //buttons: Buttons,
    //leds: Leds,
}

impl Board {
    

    fn new() -> Self {

        // no one can interrupt is when we're intializing the board
        cortex_m::interrupt::free(|cs| {

            let hstdout = HSTDOUT.borrow(cs);

            if let Some(hstdout) = hstdout.borrow_mut().as_mut() {
                writeln!(*hstdout, "Board Initialization").ok();
            }

            // initializes interrupts
            let mut cp = cortex_m::Peripherals::take().unwrap();

                //enable interrupts
            cp.NVIC.enable(nrf51::Interrupt::GPIOTE);
            cp.NVIC.clear_pending(nrf51::Interrupt::GPIOTE);
            
            let p = nrf51::Peripherals::take().unwrap(); // todo don't unwrap

            // lets borrow Peripherals
            //*GPIO.borrow(cs).borrow_mut() = Some(p.GPIOTE);
            *PERIPH.borrow(cs).borrow_mut() = Some(p);
        });

        //buttons.setup();
        Led::init();
        LEDS[0].on();
        Button::init();


        Board {
   
        }
    }
}


pub struct Led {
    i: usize,
}

impl Led {
    fn init() {
        cortex_m::interrupt::free(|cs| {
            if let Some(p) = PERIPH.borrow(cs).borrow().as_ref() {
                
                // the leds are stored in the static LEDS
                // initialize the leds pins
                for led in &LEDS {
                    p.GPIO.pin_cnf[led.i].write(|w| w.dir().output());
                }
            }

            // turn off all the leds on init
            for led in &LEDS {
                led.off();
            }
        });
    }

    fn on(&self) {
        cortex_m::interrupt::free(|cs| {
            if let Some(p) = PERIPH.borrow(cs).borrow().as_ref() {
                p.GPIO.outclr.write(|w| unsafe { w.bits(1 << self.i) });
            }
        });
    }

    fn off(&self) {
        cortex_m::interrupt::free(|cs| {
            if let Some(p) = PERIPH.borrow(cs).borrow().as_ref() {
                p.GPIO.outset.write(|w| unsafe { w.bits(1 << self.i) });
            }
        });
    }

    fn toggle(&self) {
        cortex_m::interrupt::free(|cs| {
            if let Some(p) = PERIPH.borrow(cs).borrow().as_ref() {
                    self.off();
            }
        });
    }

}


pub struct Button {
    i: usize,
}

impl Button {
    fn init() {
        cortex_m::interrupt::free(|cs| {
            if let Some(p) = PERIPH.borrow(cs).borrow().as_ref() {
                
                //configure pins in pull up input mode
                //todo change this to a macro?
                for button in &BUTTONS {
                    p.GPIO.pin_cnf[button.i].write(|w| {
                                w.dir()
                                    .input()
                                    .drive()
                                    .s0s1()
                                    .pull()
                                    .pullup()
                                    .sense()
                                    .disabled()
                                    .input()
                                    .connect()
                            });
                }

                p.GPIOTE.config[0].write(|w| unsafe { w.mode().event().psel().bits(17).polarity().lo_to_hi()});
                p.GPIOTE.intenset.write(|w| w.in0().set_bit());
                p.GPIOTE.events_in[0].write(|w| unsafe { w.bits(0) });

                p.GPIOTE.config[1].write(|w| unsafe { w.mode().event().psel().bits(19).polarity().lo_to_hi()});
                p.GPIOTE.intenset.write(|w| w.in1().set_bit());
                p.GPIOTE.events_in[1].write(|w| unsafe { w.bits(0) });

                p.GPIOTE.config[2].write(|w| unsafe { w.mode().event().psel().bits(20).polarity().lo_to_hi()});
                p.GPIOTE.intenset.write(|w| w.in2().set_bit());
                p.GPIOTE.events_in[2].write(|w| unsafe { w.bits(0) });

                p.GPIOTE.config[3].write(|w| unsafe { w.mode().event().psel().bits(21).polarity().lo_to_hi()});
                p.GPIOTE.intenset.write(|w| w.in3().set_bit());
                p.GPIOTE.events_in[3].write(|w| unsafe { w.bits(0) });
            }


            LEDS[1].on();

        });

        
    }
}

interrupt!(GPIOTE, handle_button_interrupt);

fn handle_button_interrupt() {
    /* Enter critical section */
    cortex_m::interrupt::free(|cs| {
        if let Some(p) = PERIPH.borrow(cs).borrow().as_ref() {

            let button1 = p.GPIOTE.events_in[0].read().bits() != 0;
            let button2 = p.GPIOTE.events_in[1].read().bits() != 0;
            let button3 = p.GPIOTE.events_in[2].read().bits() != 0;
            let button4 = p.GPIOTE.events_in[3].read().bits() != 0;



            let hstdout = HSTDOUT.borrow(cs);
            if let Ok(fd) = hio::hstdout() {
                *hstdout.borrow_mut() = Some(fd);
            }

            if let Some(hstdout) = hstdout.borrow_mut().as_mut() {
                writeln!(*hstdout, "button1 {}\nbutton2 {}\nbutton3 {}\nbutton4 {}", button1, button2, button3, button4).ok();
            }

            //clear button 1 events
            p.GPIOTE.events_in[0].write(|w| unsafe { w.bits(0) });
            p.GPIOTE.events_in[1].write(|w| unsafe { w.bits(0) });
            p.GPIOTE.events_in[2].write(|w| unsafe { w.bits(0) });
            p.GPIOTE.events_in[3].write(|w| unsafe { w.bits(0) });

        }
    });
}
