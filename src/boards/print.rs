extern crate cortex_m_semihosting;

use core::fmt::Write;
use cortex_m_semihosting::hio;

pub fn print(message: &str) {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "{}", message).unwrap();
}
