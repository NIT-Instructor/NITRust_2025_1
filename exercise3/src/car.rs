use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct CarError;

impl fmt::Display for CarError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid car operation")
    }
}

impl Error for CarError {}

pub struct CarStruct {
    fuel_level: f32,
    is_running: bool,
}

impl CarStruct {
    pub fn new(fuel_level: f32, is_running: bool) -> Result<Self, CarError> {
        if fuel_level < 0.0 || fuel_level > 100.0 || is_running {
            return Err(CarError);
        }
        Ok(CarStruct { fuel_level, is_running })
    }

    pub fn refuel(&mut self, amount: f32) {
        self.fuel_level = (self.fuel_level + amount).min(100.0);
    }

    pub fn start(&mut self) -> Result<(), CarError> {
        if self.is_running {
            return Err(CarError);
        }
        if self.fuel_level <= 10.0 {
            return Err(CarError);
        }
        self.is_running = true;
        Ok(())
    }

    pub fn stop(&mut self) {
        if self.is_running {
            self.is_running = false;
        }
    }
}
