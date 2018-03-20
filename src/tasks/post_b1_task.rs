use boards::print::{print};
use Semaphore;
use os_post;

pub fn post_b1(_: Option<Semaphore>) {
    print("task posting button1");

    return os_post(Semaphore::Button1);
}
