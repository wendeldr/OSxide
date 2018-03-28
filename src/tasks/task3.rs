use boards::board::Board;
use boards::nrf51dk::{Nrf51dk};
use boards::print::{print};
use Semaphore;
use os_wait;

pub fn task3(maybe_sem: Option<Semaphore>) {
    if let Some(sem) = maybe_sem {
        //print("semaphore from 3");
        let board: Nrf51dk = Nrf51dk::new();
        match sem {
            Semaphore::Button3 => board.led_toggle(2),
            _ => ()
        }
    }

    //print("now waiting from 3");
    return os_wait(Semaphore::Button3);
}
