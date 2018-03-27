
use cortex_m::asm::bkpt;
use cortex_m;

use nrf51;
use boards::nrf51dk::PERIPH;
use boards::print::{print};

#[derive(PartialEq, Clone)]
pub enum PendingInterrupt {
    Button1,
    Button2,
    Button3,
    Button4
}

pub struct Interrupt {
    pub pending_interrupt: Option<PendingInterrupt>
}

impl Interrupt {

    pub fn new() -> Interrupt {
        Interrupt { pending_interrupt: None }
    }

    #[allow(non_snake_case)]
    pub fn GPIOTE_IRQHandler(&mut self) {
        cortex_m::interrupt::free(|cs| {
            if let Some(p) = PERIPH.borrow(cs).borrow().as_ref() {

                // TODO we should be referencing the buttons array
                for i in 0..4 {
                     let button = p.GPIOTE.events_in[i].read().bits() != 0;

                     if button {
                        //print("button pressed");
                        self.set_button_pending_interrupt(i);
                     }

                     // clear the events
                     p.GPIOTE.events_in[i].write(|w| unsafe { w.bits(0) });
                }
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

    fn set_button_pending_interrupt(&mut self, button_index: usize) {
        let button_interrupts = [
            PendingInterrupt::Button1,
            PendingInterrupt::Button2,
            PendingInterrupt::Button3,
            PendingInterrupt::Button4
        ];

        self.pending_interrupt = Some(button_interrupts[button_index].clone());
    }


    pub fn consume_if_pending_interrupt(&mut self) -> Option<PendingInterrupt> {
        let mut maybe_interrupt = None;

        cortex_m::interrupt::free(|_cs| {
            if let Some(ref interrupt) = self.pending_interrupt {
                maybe_interrupt = Some(interrupt.clone());
            }

            self.pending_interrupt = None;
        });

        return maybe_interrupt;
    }
    
    #[allow(non_snake_case)]
    pub fn TIMER1_IRQHandler() {
        cortex_m::interrupt::free(|_cs| {
                /*Do Something*/

            unsafe {
                // clear the register
                (*nrf51::TIMER1::ptr()).tasks_clear.write(|w| w.bits(1));
                // clear the event
                (*nrf51::TIMER1::ptr()).events_compare[1].write(|w| w.bits(0));
            }
            bkpt();
        });
    }

    #[allow(non_snake_case)]
    pub fn TIMER2_IRQHandler() {
        cortex_m::interrupt::free(|_cs| {
                /*Do Something*/

            unsafe {
                // clear the register
                (*nrf51::TIMER2::ptr()).tasks_clear.write(|w| w.bits(1));
                // clear the event
                (*nrf51::TIMER2::ptr()).events_compare[2].write(|w| w.bits(0));
            }
            bkpt();
        });
    }
}