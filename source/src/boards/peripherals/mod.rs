pub mod buttons;
pub mod leds;
pub mod timers;
pub mod gpio;

/* 
    TODO fix this
    This exposes a statically assigned singleton for maniuplating
    gpio, honestly this is disgusting way to do this
*/
use super::PERIPH;