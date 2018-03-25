

use cortex_m::asm::bkpt;

pub struct Interrupt {

}

impl Interrupt {
    pub fn GPIOTE_IRQHandler() {
        bkpt();
    }

    pub fn TIMER0_IRQHandler() {
        bkpt();
    }
}