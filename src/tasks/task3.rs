use boards::print::{print};
use Semaphore;
use os_wait;

pub fn task3(maybe_sem: Option<Semaphore>) {
    if let Some(_) = maybe_sem {
        print("hey a sem");
    }
    print("called from task 3");
    return os_wait(Semaphore::Button3);
}
