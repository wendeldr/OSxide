use boards::print::{print};
use Semaphore;
use os_wait;

pub fn task1(maybe_sem: Option<Semaphore>) {
    if let Some(_) = maybe_sem {
        print("hey a sem");
    }
    print("called from task 1");

    return os_wait(Semaphore::Button1);
}


