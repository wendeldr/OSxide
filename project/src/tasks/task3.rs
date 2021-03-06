use boards::board::Board;
use boards::nrf51dk::{Nrf51dk};
use Semaphore;
use Kernel;

pub fn task3(maybe_sem: Option<Semaphore>) {
    if let Some(sem) = maybe_sem {
        let board: Nrf51dk = Nrf51dk::new();
        match sem {
            Semaphore::Button3 => {
                                    for _i in 0..4 {
                                        board.led_toggle(0);
                                        Kernel::os_sleep(200);
                                        board.led_toggle(1);
                                        board.led_toggle(0);
                                        Kernel::os_sleep(200);
                                        board.led_toggle(3);
                                        board.led_toggle(1);
                                        Kernel::os_sleep(200);
                                        board.led_toggle(2);
                                        board.led_toggle(3);
                                        Kernel::os_sleep(200);
                                        board.led_toggle(2);
                                    }
                                },
            _ => ()
        }
    }

    //print("now waiting from 3");
    return Kernel::os_wait(Semaphore::Button3);
}
