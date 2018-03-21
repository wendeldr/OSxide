
#[allow(non_camel_case_types)]


pub trait Board {
     
     /* Returns self */
     fn new( ) -> Self;
     
     /* Turns on the led specfied by i */
     fn led_on(&self, i: usize);

     /* Turns off the led specified by i */
     fn led_off(&self, i: usize);
}