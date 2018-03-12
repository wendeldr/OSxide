
#![feature(used)]
#![no_std]
#![feature(asm)]  // for `bkpt!`


mod boards;

use boards::board::Board;
use boards::nrf51dk::nrf51dk;
use boards::nrf51dk::cortex_m_semihosting::hio;

use core::fmt::Write;


fn main() {

    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Main Loop").unwrap();

    // Import is a little jacked up. this should be its own crate
    let mut board : nrf51dk = Board::new("nrf51dk");

    board.init();


    loop {
        // handle interrupt

    }


    
}