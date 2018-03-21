

extern crate cortex_m;

use super::PERIPH;

use boards::nrf51dk::LEDS;


pub struct Led {
    pub i: usize,
}

impl Led {
    pub fn init()  {
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

    pub fn on(&self) {
        cortex_m::interrupt::free(|cs| {
            if let Some(p) = PERIPH.borrow(cs).borrow().as_ref() {
                p.GPIO.outclr.write(|w| unsafe { w.bits(1 << self.i) });
            }
        });
    }

    pub fn off(&self) {
        cortex_m::interrupt::free(|cs| {
            if let Some(p) = PERIPH.borrow(cs).borrow().as_ref() {
                p.GPIO.outset.write(|w| unsafe { w.bits(1 << self.i) });
            }
        });
    }

    fn toggle(&self) {
        cortex_m::interrupt::free(|cs| {
            if let Some(_p) = PERIPH.borrow(cs).borrow().as_ref() {
                    self.off();
            }
        });
    }

}