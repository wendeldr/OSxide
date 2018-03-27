use boards::board::Board;
use boards::nrf51dk::{Nrf51dk};
use boards::print::{print};
use Semaphore;
use os_wait;

pub fn task1(maybe_sem: Option<Semaphore>) {
    if let Some(sem) = maybe_sem {
        let board: Nrf51dk = Nrf51dk::new();
        match sem {
            Semaphore::Button1 => board.led_toggle(0),
            _ => ()
        }
    }

    return os_wait(Semaphore::Button1);
}


