

extern crate cortex_m;

use super::PERIPH;

use boards::nrf51dk::BUTTONS;

pub struct Button {
    pub i: usize,
}

impl Button {
    pub fn init() {
        cortex_m::interrupt::free(|cs| {
            if let Some(p) = PERIPH.borrow(cs).borrow().as_ref() {
                
                //configure pins in pull up input mode
                //todo change this to a macro?
                let mut i: usize = 0;
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

                    p.GPIOTE.config[i].write(|w| unsafe { w.mode().event().psel().bits(button.i as u8).polarity().lo_to_hi()});
                    p.GPIOTE.events_in[i].write(|w| unsafe { w.bits(0) });
                    i += 1;
                }

                p.GPIOTE.intenset.write(|w| w.in0().set_bit());
                p.GPIOTE.intenset.write(|w| w.in1().set_bit());
                p.GPIOTE.intenset.write(|w| w.in2().set_bit());
                p.GPIOTE.intenset.write(|w| w.in3().set_bit());
            }
        });
    }
}