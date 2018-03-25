
#![feature(used)]
#![feature(linkage)]
#![feature(naked_functions)]
#![feature(core_intrinsics)] 
#![feature(asm)]  // for `bkpt!`
#![allow(dead_code)]
#![no_std]

#[macro_use(interrupt)]


extern crate nrf51;
extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate cortex_m_rt;
//use cortex_m::asm::bkpt;

// INTERNAL MODS
//mod chips;

mod boards;
//use boards::interrupt;
use boards::board::{Board};
use boards::nrf51dk::{Nrf51dk};
use boards::interrupt::Interrupt;

mod tasks;



use tasks::*;


// TYPE DEFS
#[derive(PartialEq, Clone)]
pub enum Semaphore {
    Button1,
    Button2,
    Button3,
    Button4
}

struct TaskControlBlock {
    sem: Option<Semaphore>,
    task: fn(Option<Semaphore>) -> ()
}

// OS GLOBALS
const NUM_TASKS: usize = 4;
static mut OS_SEM: Option<Semaphore> = None;
static mut CURR_TID: usize = NUM_TASKS - 1;
static mut OS_TASKS: [TaskControlBlock; NUM_TASKS] = [
    TaskControlBlock { sem: None, task: post_b1_task::post_b1 },
    TaskControlBlock { sem: None, task: task1::task1 },
    TaskControlBlock { sem: None, task: task2::task2 },
    TaskControlBlock { sem: None, task: task3::task3 }
];


// OS OPERATIONS
fn os_switch() {
    for tid in (0..NUM_TASKS).rev() {
        unsafe {
            let task_in_question = &OS_TASKS[tid as usize];
            // if the task in question is waiting on something
            if let Some(ref task_sem) = task_in_question.sem {
                // if an sempahore has been posted
                if let Some(ref os_sem) = OS_SEM {
                    // if the posted semaphore is being waited on
                    // by the task in question
                    if *os_sem == *task_sem {
                        // task in question is no longer waiting on a sem
                        OS_TASKS[tid] =
                            TaskControlBlock { sem: None, ..*task_in_question };
                        CURR_TID = tid;
                        let os_sem_clone = os_sem.clone();
                        OS_SEM = None; // TODO should we leave the sem?
                        return (task_in_question.task)(Some(os_sem_clone));
                    }
                }
            } else {
                CURR_TID = tid;
                return (task_in_question.task)(None);
            }
        }
    }
}

pub fn os_wait(sem: Semaphore) {
    unsafe {
        let curr_task = &OS_TASKS[CURR_TID];
        OS_TASKS[CURR_TID] = TaskControlBlock { sem: Some(sem), ..*curr_task };
    }
    return os_yeild();
}

pub fn os_post(sem: Semaphore) {
    // TODO should this be critical?
    unsafe {
        OS_SEM = Some(sem)
    }
    os_yeild();
}

pub fn os_yeild() {
    return os_switch();
}

fn main() {
    let board: Nrf51dk = Nrf51dk::new();
    board.init();
    board.led_on(1);
    os_switch();
}


// For some reason I can only hit this in the main file
interrupt!(GPIOTE, Interrupt::GPIOTE_IRQHandler);