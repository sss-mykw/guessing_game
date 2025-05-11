pub struct Guess {
    value: u32,
}

#[derive(Debug)]
pub enum RangeError {
    OutOfRange { value: u32, min: u32, max: u32 },
}

impl Guess {
    pub fn new(value: u32) -> Result<Guess, RangeError> {
        let min = 1;
        let max = 100;
        if (min..=max).contains(&value) { 
            Ok(Guess { value })
        } else {
            Err(RangeError::OutOfRange { value, min, max })
        }
    }
    
    pub fn value(&self) -> u32 {
        self.value
    }
}
