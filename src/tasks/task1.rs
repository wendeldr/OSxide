use boards::board::Board;
use boards::nrf51dk::{Nrf51dk};
use boards::print::{print};
use Semaphore;
use os_wait;

pub fn task1(maybe_sem: Option<Semaphore>) {
    if let Some(sem) = maybe_sem {
        print("semaphore from 1");
        let board: Nrf51dk = Nrf51dk::new();
        match sem {
            Semaphore::Button1 => board.led_on(0),
            _ => ()
        }
    }

    print("now waiting from 1");
    return os_wait(Semaphore::Button1);
}


