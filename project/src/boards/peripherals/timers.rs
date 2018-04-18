extern crate cortex_m;

use nrf51::{self, timer0};

#[derive(Clone, Copy)]
pub struct Timer {
    pub timer: *const timer0::RegisterBlock,
    pub prescaler: u8,
    pub i: usize,
}

impl Timer {

    pub fn new(i: usize, frequency: u32) -> Timer {

        let timer = match i {
            0 => nrf51::TIMER0::ptr(),
            1 => nrf51::TIMER1::ptr(),
            2 => nrf51::TIMER2::ptr(),
            _ => panic!("Invalid Timer: {}", i),
        };

        let prescaler = match frequency {
            16000000 => 0,
            8000000 => 1,
            4000000 => 2,
            2000000 => 3,
            1000000 => 4,
            500000 => 5,
            250000 => 6,
            125000 => 7,
            62500 => 8,
            31250 => 9,
            _ => panic!("Invalid frequency: {}", frequency),
        };

        Timer {
            timer,
            prescaler,
            i,
        }
    }

    pub fn init(&self, us: u32) {

        // lets make sure we don't get interrupted..
        cortex_m::interrupt::free(|_cs| {
            unsafe {
                (*self.timer).prescaler.write(|w| w.prescaler().bits(self.prescaler));
                (*self.timer).bitmode.write(|w| w.bitmode()._32bit());
                //Set the Timer intent
                (*self.timer).intenset.write(|w| w.compare0().set());
                // self.i contains the config iterator
                (*self.timer).events_compare[self.i].write(|w| w.bits(0));
                /* Program counter compare register with value */
                (*self.timer).cc[self.i].write(|w| w.bits(us) );
            }
        });

        //clear out the register
        self.clear();

    }

    pub fn start(&self) {
        self.resume();
    }

    pub fn resume(&self) {
        cortex_m::interrupt::free(|_cs| {
            unsafe {
                (*self.timer).tasks_start.write(|w| w.bits(1));
            }
        });
    }

    pub fn pause(&self) {
        cortex_m::interrupt::free(|_cs| {
            unsafe {
                (*self.timer).tasks_stop.write(|w| w.bits(1));
            }
        });
    }

    pub fn clear(&self) {
        cortex_m::interrupt::free(|_cs| {
            unsafe {
                (*self.timer).tasks_clear.write(|w| w.bits(1));
            }
        });
    }

    pub fn clear_event(&self) {
        cortex_m::interrupt::free(|_cs| {
            unsafe {
                (*self.timer).events_compare[self.i].write(|w| w.bits(0));
            }
        });
    }

    pub fn get_value(&self) -> u32 {
        cortex_m::interrupt::free(|_cs| -> u32 {
            
            unsafe {
                // capture the time value and store in cc
                (*self.timer).tasks_capture[self.i].write(|w| w.bits(0));
                 // return time passed
                 (*self.timer).cc[self.i].read().bits()
            }
        })


    }
}