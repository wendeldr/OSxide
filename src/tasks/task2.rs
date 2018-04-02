use boards::board::Board;
use boards::nrf51dk::{Nrf51dk};
use boards::print::{print};
use Semaphore;
use Kernel;

pub fn task2(maybe_sem: Option<Semaphore>) {
    if let Some(sem) = maybe_sem {
        //print("semaphore from 2");
        let board: Nrf51dk = Nrf51dk::new();
        match sem {
            Semaphore::Button2 => {
                                    for _i in 0..4 {
                                        board.led_toggle(0);
                                        Kernel::os_sleep(200);
                                        board.led_toggle(1);
                                        Kernel::os_sleep(200);
                                        board.led_toggle(2);
                                        Kernel::os_sleep(200);
                                        board.led_toggle(3);
                                        Kernel::os_sleep(200);
                                    }
                                },
            _ => ()
        }
    }

    //print("now waiting from 2");
    return Kernel::os_wait(Semaphore::Button2);
}
