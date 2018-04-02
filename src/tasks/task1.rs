use boards::board::Board;
use boards::nrf51dk::{Nrf51dk};
use boards::print::{print};
use Semaphore;
use Kernel;

use cortex_m;

pub fn task1(maybe_sem: Option<Semaphore>) {
    if let Some(sem) = maybe_sem {
        //print("semaphore from 1");
        let board: Nrf51dk = Nrf51dk::new();
        match sem {
            Semaphore::Button1 => {
                            for i in 0..10 {
                                board.led_toggle(0);
                                Kernel::os_sleep(200);
                                board.led_toggle(1);
                            }
                        },
            _ => ()
        }
    }

    //print("now waiting from 1");
    return Kernel::os_wait(Semaphore::Button1);
}


