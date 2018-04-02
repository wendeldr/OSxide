
use nrf51;
use cortex_m;
use cortex_m::asm::bkpt;


pub struct Gpio {
    register_block: *const nrf51::gpio::RegisterBlock,
    pin: usize
}

impl Gpio {

    pub fn new(pin: usize) -> Self {
        // get a reference to the gpio block
        let register_block = nrf51::GPIO::ptr();

        Gpio {
            register_block,
            pin
        }
    }

    fn register_block(&self) -> *const nrf51::gpio::RegisterBlock {
        self.register_block
    }

    fn pin(&self) -> usize {
        self.pin
    }

    // function methods
    pub fn set_pull_up(&self) {
        cortex_m::interrupt::free(|_cs| {
            unsafe {
                (*self.register_block)
                    .pin_cnf[self.pin]
                    .write(|w| {
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
        });
    }


    pub fn set_output(&self) {
        cortex_m::interrupt::free(|_cs| {
            unsafe {
                (*self.register_block)
                    .pin_cnf[self.pin]
                    .write(|w| w.dir().output());
            }
        });
    }

    pub fn is_high(&self) -> bool {
        cortex_m::interrupt::free(|_cs| -> bool {
            unsafe {
                (*self.register_block).in_.read().bits() & (1 << self.pin) == 0
            }
        })
    }

    pub fn clear_register(&self) {
        cortex_m::interrupt::free(|_cs| {
            unsafe {
                (*self.register_block).outclr.write(|w| w.bits(1 << self.pin));
            }
        });
    }

    pub fn set_register(&self) {
        cortex_m::interrupt::free(|_cs| {
            unsafe {
                (*self.register_block).outset.write(|w| w.bits(1 << self.pin));
            }
        });
    }
}

pub struct Led {
    gpio: Gpio,
}

impl Led {
    pub fn new(pin: usize) -> Self {
        // get a reference to the gpio block

        let led = Gpio::new(pin);

        led.set_output();

        Led {
            gpio: led
        }
    }

    pub fn on(&self) {
        self.gpio.clear_register();
    }

    pub fn off(&self) {
        self.gpio.set_register();
    }

    pub fn toggle(&self) {
        let state: bool = self.gpio.is_high();
        
        if state {
            self.off();
        }  else {
            self.on();
        }
    }
}

pub struct Button {
    gpio: Gpio
}

impl Button {
    pub fn new(pin: usize) -> Self {
        // get a reference to the gpio block

        let button = Gpio::new(pin);

        Button {
            gpio: button
        }
    }


    pub fn set_pull_up(&self) {
        self.gpio.set_pull_up();
    }

    pub fn is_high(&self) -> bool {
        self.gpio.is_high()
    }
}