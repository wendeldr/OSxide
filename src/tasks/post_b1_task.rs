use boards::print::{print};
use Semaphore;
use Kernel;

pub fn post_b1(_: Option<Semaphore>) {
    print("task posting button1");

    return Kernel::os_post(Semaphore::Button1);
}
