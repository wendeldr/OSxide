
pub mod nrf51dk;
pub mod board;
pub mod print;
pub mod peripherals;

/* 
    TODO fix this
    This exposes a statically assigned singleton for maniuplating
    gpio, honestly this is disgusting way to do this
*/
pub use self::nrf51dk::PERIPH;