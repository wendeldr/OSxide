extern crate nrf51;

use self::nrf51::{TIMER0};


pub struct Timer {
    timer: TIMER0,
}

impl Timer {

    fn new(timer: TIMER0) -> Self {
        Timer { timer }
    }

    fn start(&mut self) {
        // todo
        self.timer.
    }

    fn stop(&mut self) {
        // todo
    }

    fn set(&mut self, interval: u32) {

    }

    fn enable_interrupt(&mut self) {
        self.timer.intenset.write(|w| w.compare0().set());
    }

    fn disable_interrupt(&mut self) {
        self.timer.intenclr.write(|w| w.compare0().clear());
    }

}

