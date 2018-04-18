# User Manual 

*****

## Introduction

O<sup>s</sup>xide is a real time operating system written in Rust that aims create a foundation for secure Internet of Things (IOT) applications. Osxide targets a Cortex M0, specifically the Nordic NRF51 Development Kit.  

*****

## Installing

Prerequisites: Rust, Linux

```bash
# Step 1 Install rust nightly build
$ rustup default nightly

# Step 2 Install the ARM linker and utilities
$ sudo apt-get install binutils-arm-none-eabi

# Step 3 Install arm gdb
$ sudo apt-get install gdb-arm-none-eabi

# Step 4 Install xargo
$ cargo install xargo
```



*****

## Writing Tasks

Currently tasks are written in rust and added to a list of processes the kernel loops over and schedules. Below is a task function that gets a reference to the Board which gives access to peripherals. The Kernel also exposes kernel functions like sleep which tells the current process to do nothing for a desired amount of time.

This file should be added to the tasks folder under the src/ directory.

Example src/task/task1.rs:

```rust
use boards::board::Board;
use boards::nrf51dk::{Nrf51dk};
use Semaphore;
use Kernel;

pub fn task1(maybe_sem: Option<Semaphore>) {
    if let Some(sem) = maybe_sem {
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



Next, add expose the task via the mod.rs file in the same folder.

The syntax should look similar to 

```rust
pub mod task1;
```



Finally, in main.rs add the function to the static array

```rust
static mut OS_TASKS: [TaskControlBlock; NUM_TASKS] = [
    TaskControlBlock { sem: None, start_time: 0, task: Kernel::os_idle_task },
    TaskControlBlock { sem: None, start_time: 0, task: task1::task1 },
    TaskControlBlock { sem: None, start_time: 0, task: task2::task2 },
    TaskControlBlock { sem: None, start_time: 0, task: task3::task3 }
];
```

*****

## Building and Flashing

The project includes two OCD Config file that provides board flashing capability.

### Development

For development purposes, in two seperate terminals do the following.

Terminal 1

```bash
# Root directory of Osxide
$ openocd -f configs/nrf51dk.cfg
```



Terminal 2

```bash
# Root directory of Osxide
$ xargo run
```



This will open a gdb server on the board over JLINK and allow for gdb usage.

### Production

```bash
# Root directory of Osxide
$ xargo build --release
$ ./flash_device.sh target/thumbv6m-none-eabi/release/rtos 
```

This will flash the device with the desired binary using the config files provided in release mode which is much smaller than the debug binary.

*****

## FAQ

**Q** "What is a real time operating system?"

**A** A real time operating system is a system designed to execute and complete tasks in a foreseeable time frame. Often times this means reacting to real world events without the overhead of a general purpose operating system.

**Q** How is rust safer than C?

While C is a wonderful language and has been used for well over three decades, unsafe C can easily written and even now we experience many flaws that were introduced at the conception of the language due to how closely to the hardware you are working with. Rust attempts to correct this through the use of ownership in regards to variables and pointers. Meaning it becomes a lot easier to write concurrent safe rust. In fact to quote the rust nomicon, 

> "Safe Rust is the *true* Rust programming language. If all you do is write Safe Rust, you will never have to worry about type-safety or memory-safety. You will never endure a dangling pointer, a use-after-free, or any other kind of Undefined Behavior."

However rust can also be unsafe and allow for many things that C handles but when an issue arises it becomes infinity easier to detect through the required use of unsafe blocks. For example and unsafe portion would be interacting with the peripherals on the board since we have to dereference a pointer we don't own and shift bits.

**Q** "Is your project an operating system to rust things with oxygen?"

**A** No. 

**Q** "Does your project catch fire?"

**A** No.