
#![feature(used)]
#![feature(linkage)]
#![feature(naked_functions)]
#![feature(core_intrinsics)] 
#![feature(asm)]  // for `bkpt!`
#![no_std]

extern crate cortex_m_semihosting;
extern crate cortex_m;

use cortex_m::interrupt::{self, Mutex};

mod chips;
mod tasks;
mod boards;

use chips::chip::{Chip};
use chips::nrf51xxx::{NRF51};
use tasks::task1::{task1};


pub enum Semaphore {
    Button1,
    Button2,
    Button3,
    Button4
}

struct TaskControlBlock {
    tid: u32,
    sem: Option<Semaphore>,
    task: fn(Semaphore) -> ()
}

static OS_SEM: Option<Semaphore> = None;
static CURR_TID: u32 = 0;
static OS_TASKS: [TaskControlBlock; 1] = [
    TaskControlBlock { tid: 1, sem: None, task: task1}
];

fn main() {
    let mut chip: NRF51 = NRF51::new();
    chip.write("Main Loop");
    chip.init();
    let tasks = (OS_TASKS[0].task)(Semaphore::Button1);
}