

pub struct microbit { name: &'static str }

impl microbit {
    pub fn init(&self) {
        // todo implement
    }

    pub fn service_pending_interrupt() {
        //todo implementation
    }
}

impl Board for microbit {
    fn new( ) -> microbit {
        microbit { }
    }

    
}