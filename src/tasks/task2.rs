use boards::board::Board;
use boards::nrf51dk::{Nrf51dk};
use boards::print::{print};
use Semaphore;
use os_wait;

pub fn task2(maybe_sem: Option<Semaphore>) {
    if let Some(sem) = maybe_sem {
        print("semaphore from 2");
        let board: Nrf51dk = Nrf51dk::new();
        match sem {
            Semaphore::Button2 => board.led_toggle(1),
            _ => ()
        }
    }

    print("now waiting from 2");
    return os_wait(Semaphore::Button2);
}
