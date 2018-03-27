
use cortex_m::asm::bkpt;
use cortex_m;

use nrf51;
use boards::nrf51dk::PERIPH;

use Semaphore;

pub struct Interrupt {

}

impl Interrupt {

    #[allow(non_snake_case)]
    pub fn GPIOTE_IRQHandler() {
        cortex_m::interrupt::free(|cs| {
            if let Some(p) = PERIPH.borrow(cs).borrow().as_ref() {

                // TODO we should be referencing the buttons array
                
                for i in 0..4 {
                     let _button = p.GPIOTE.events_in[i].read().bits() != 0;
                     
                     /* DO something*/

                     // clear the events
                     p.GPIOTE.events_in[i].write(|w| unsafe { w.bits(0) });
                }
                //bkpt();

            }
        });
    }

    #[allow(non_snake_case)]
    pub fn TIMER0_IRQHandler() {
        cortex_m::interrupt::free(|_cs| {
                /*Do Something*/

            unsafe {
                // clear the register
                (*nrf51::TIMER0::ptr()).tasks_clear.write(|w| w.bits(1));
                // clear the event
                (*nrf51::TIMER0::ptr()).events_compare[0].write(|w| w.bits(0));
            }
            bkpt();
        });
    }
}