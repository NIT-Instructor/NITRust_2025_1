struct Battery {
    charge_level: u32,
    is_disposed: bool,
}

impl Battery {
    fn use_up(mut self) {
        self.charge_level = 0;
        self.is_disposed = true;
        println!("Battery fully discharged and disposed.");
    }

    fn dispose(mut self) {
        if !self.is_disposed {
            self.is_disposed = true;
            println!("Battery disposed.");
        } else {
            println!("Battery is already disposed.");
        }
    }
}

fn main() {
    let battery1 = Battery {
        charge_level: 100,
        is_disposed: false,
    };

    let battery2 = Battery {
        charge_level: 50,
        is_disposed: false,
    };

    let battery3 = Battery {
        charge_level: 75,
        is_disposed: true,
    };

    battery1.dispose();
    battery2.use_up();
    battery3.dispose();

    // battery1.use_up(); // This will cause a compile-time error, as battery1 has been moved in dispose method.
    // battery2.dispose(); // This would cause a compile-time error, as battery2 has been moved in use_up method.
}