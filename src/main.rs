
#![feature(used)]
#![feature(linkage)]
#![feature(naked_functions)]
#![feature(core_intrinsics)] 
#![feature(asm)]  // for `bkpt!`
#![no_std]
extern crate cortex_m_semihosting;

mod chips;

use chips::chip::{Chip};
use chips::nrf51xxx::{NRF51};
//use boards::board::Board;
//use boards::nrf51dk::nrf51dk;
//use boards::nrf51dk::


fn main() {

    let mut chip: NRF51 = Chip::new();
    chip.write("Main Loop");
    chip.init();


    loop {
        // handle interrupt

    }


    
}