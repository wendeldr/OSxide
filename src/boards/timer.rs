
extern crate nrf51;
extern crate cortex_m;

use self::cortex_m_semihosting::hio::{self, HStdout};
use self::cortex_m::interrupt::{Mutex};

use self::nrf51::interrupt;

static HSTDOUT: Mutex<RefCell<Option<HStdout>>> = Mutex::new(RefCell::new(None));


interrupt!(TIMER0, Timer::timer0_irqhandler);
interrupt!(TIMER1, Timer::timer1_irqhandler);
interrupt!(TIMER2, Timer::timer2_irqhandler);





struct Timer {
}

impl Timer {

    


    fn timer0_irqhandler() {
        interrupt::free(|cs| {
            let hstdout = HSTDOUT.borrow(cs);
            if let Some(hstdout) = hstdout.borrow_mut().as_mut() {
                writeln!(*hstdout, "Tock0").ok();
            }
        });
    }

    fn timer1_irqhandler() {
         interrupt::free(|cs| {
            let hstdout = HSTDOUT.borrow(cs);
            if let Some(hstdout) = hstdout.borrow_mut().as_mut() {
                writeln!(*hstdout, "Tock1").ok();
            }
        });
    }

    fn timer2_irqhandler() {
         interrupt::free(|cs| {
            let hstdout = HSTDOUT.borrow(cs);
            if let Some(hstdout) = hstdout.borrow_mut().as_mut() {
                writeln!(*hstdout, "Tock2").ok();
            }
        });
    }
}