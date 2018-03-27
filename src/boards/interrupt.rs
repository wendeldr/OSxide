
use cortex_m::asm::bkpt;
use cortex_m;

use boards::nrf51dk::PERIPH;
use boards::print::{print};

pub struct Interrupt {

}

impl Interrupt {

    #[allow(non_snake_case)]
    pub fn GPIOTE_IRQHandler() {
        cortex_m::interrupt::free(|cs| {
            if let Some(p) = PERIPH.borrow(cs).borrow().as_ref() {

                // TODO we should be referencing the buttons array
                
                for i in 0..4 {
                     let button = p.GPIOTE.events_in[i].read().bits() != 0;

                     if button {
                        print("button");
                     }

                     p.GPIOTE.events_in[i].write(|w| unsafe { w.bits(0) });
                }
                bkpt();

            }
        });
    }

    #[allow(non_snake_case)]
    pub fn TIMER0_IRQHandler() {
        cortex_m::interrupt::free(|cs| {
            bkpt();
        });
    }
}