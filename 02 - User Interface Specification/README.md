# User Interface Specification
Table of Contents

1. [Technologies Used](##Technologies Used)

2. [Initial Setup](##Use)

3. [Use](##Use)

4. [To Setup A New Task](##To Setup A New Task)\

   ​



## Technologies Used

- Rust | https://www.rust-lang.org/en-US/install.html
- Openocd | https://github.com/ntfreak/openocd
- xargo | https://github.com/japaric/xargo
- OSxide | https://github.com/wendeldr/OSxide

##Initial Setup

###Rust

**Windows**: Download and install *rustup-init.exe* from the Rust website.

**Linux / Mac**: `curl https://sh.rustup.rs -sSf | sh`

After instillation run `rustup --channel=nightly` in console



### Openocd

For use with the nrf51 version 10+ of Openocd is required

**Windows / Linux**: Follow the *'Building OpenOCD'* section of the Openocd github link above

**Mac:** `brew install openocd`



### Xargo

Requires Rust nightly to first be installed

**All systems**:

1. `rustup component add rust-src`
2. `cargo install xargo`



###OSxide

`git clone https://github.com/wendeldr/OSxide.git`



## Use

OSxide is designed to run tasks in order based off the directory contents in *'source/src/tasks'*,  the file *source/src/tasksmod.rs*, and the task array in *'source/main.rs'*  `47. static mut OS_TASKS: [TaskControlBlock; NUM_TASKS]`.  The in this array is **always** the Kernel idle task os_idle_task which waits for semaphore changes.  The other tasks can be modified to do desired operations on the operating system.

Below is an example task

```rust
use boards::board::Board;
use boards::nrf51dk::{Nrf51dk};
use Semaphore;
use Kernel;

pub fn task1(maybe_sem: Option<Semaphore>) {
    if let Some(sem) = maybe_sem {
        //print("semaphore from 1");
        let board: Nrf51dk = Nrf51dk::new();
        match sem {
            Semaphore::Button1 => {
                            for _i in 0..10 {
                                board.led_toggle(0);
                                Kernel::os_sleep(200);
                                board.led_toggle(1);
                            }
                        },
            _ => ()
        }
    }

    //print("now waiting from 1");
    return Kernel::os_wait(Semaphore::Button1);
}
```

This task waits for the semaphore `Button1` which is posted by the OS when the button is pressed.  It then turns on and off the LEDs 10 times.

Tasks at this time can do basic calculations, modify led's, and whatever else is desired. Think of tasks as separate non-interacting applications that run all the way to completion on the OS.  The currently supported semaphores for the OS are buttons 1-4 (`Semaphore::Button1`,`Semaphore::Button2`...). 



## To setup a new task

Modify an existing task or add a new file and use this basic shell:

```rust
use boards::board::Board;
use boards::nrf51dk::{Nrf51dk};
use Semaphore;
use Kernel;

pub fn TASKNAME(maybe_sem: Option<Semaphore>) {
    if let Some(sem) = maybe_sem {
        let board: Nrf51dk = Nrf51dk::new();
        match sem {
            Semaphore::DESIRED_SEM => {
                            //Place code to occur after desired sem here
                        },
            _ => ()
        }
    }
    
    return Kernel::os_wait(Semaphore::DESIRED_SEM);
}
```

Then:

1. Modify `TASKNAME` , `DESIRED_SEM`, and place code to run where comment is.
2. Add new line `pub mod TASKNAME;` to the file *source/src/tasks/mod.rs*
3. Add new line to the task control block OS_TASKS in *source/src/main.rs*.:`TaskControlBlock { sem: None, start_time: 0, task: TASKNAME::TASKNAME }` where TASKNAME is replaced with name from before
4. Compile and run

