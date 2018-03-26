extern crate cortex_m;

use super::PERIPH;

pub struct Timer {

}

impl Timer {
    pub fn init(us: u32) {
        cortex_m::interrupt::free(|cs| {
            if let Some(p) = PERIPH.borrow(cs).borrow().as_ref() {
                // Set timer to 32 bit and prescalar to 4
                p.TIMER0.bitmode.write(|w| w.bitmode()._32bit());
                p.TIMER0.prescaler.write(|w| unsafe { w.prescaler().bits(4) });

                //Set the Timer intent
                p.TIMER0.intenset.write(|w| w.compare0().set());
                p.TIMER0.shorts.write(|w| w.compare0_clear().enabled());
                p.TIMER0.events_compare[0].write(|w| unsafe { w.bits(0) });

                /* Program counter compare register with value */
                p.TIMER0.cc[0].write(|w| unsafe { w.bits(us) });

                /* Clear current counter value */
                p.TIMER0.tasks_clear.write(|w| unsafe { w.bits(1) });

                /* Start counting */
                p.TIMER0.tasks_start.write(|w| unsafe { w.bits(1) });
            }
        });
    }

    pub fn pause() {

    }

    pub fn resume() {

    }

    pub fn clear() {

    }

    pub fn clear_event() {
        
    }
}