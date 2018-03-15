extern crate nrf51;

use core::mem;
use self::nrf51::TIMER0;

pub struct Timer{
    timer: TIMER0
}


impl Timer {
    pub fn new(timer: TIMER0) -> Self {

        //let mut timer: nrf51::TIMER0 =  mem::transmute(timer);
        Timer{ timer}
    }

    pub fn clear(&self) {

        self.timer.tasks_stop.write(|w| unsafe { w.bits(1) });
    }
}
