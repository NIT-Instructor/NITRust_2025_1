#[allow(dead_code)]
pub struct Battery {
    pub charge_level: f32,
    pub is_disposed: bool,
}

impl Battery {
    pub fn use_up(self) -> Battery {
        Battery {
            charge_level: 0.0,
            is_disposed: true,
        }
    }

    pub fn dispose(self) {
        if self.is_disposed {
            println!("Battery is already disposed.");
        } else {
            println!("Disposing battery.");
        }
    }
}
