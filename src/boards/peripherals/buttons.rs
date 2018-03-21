

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
                }


                /*
                    TODO FIX ME
                    These set the peripheral to interrupt
                    these should just be looped over like the configurations

                */
                p.GPIOTE.config[0].write(|w| unsafe { w.mode().event().psel().bits(17).polarity().lo_to_hi()});
                p.GPIOTE.intenset.write(|w| w.in0().set_bit());
                p.GPIOTE.events_in[0].write(|w| unsafe { w.bits(0) });

                p.GPIOTE.config[1].write(|w| unsafe { w.mode().event().psel().bits(18).polarity().lo_to_hi()});
                p.GPIOTE.intenset.write(|w| w.in1().set_bit());
                p.GPIOTE.events_in[1].write(|w| unsafe { w.bits(0) });

                p.GPIOTE.config[2].write(|w| unsafe { w.mode().event().psel().bits(19).polarity().lo_to_hi()});
                p.GPIOTE.intenset.write(|w| w.in2().set_bit());
                p.GPIOTE.events_in[2].write(|w| unsafe { w.bits(0) });

                p.GPIOTE.config[3].write(|w| unsafe { w.mode().event().psel().bits(20).polarity().lo_to_hi()});
                p.GPIOTE.intenset.write(|w| w.in3().set_bit());
                p.GPIOTE.events_in[3].write(|w| unsafe { w.bits(0) });
            }
        });
    }
}