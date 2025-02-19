struct Battery {
    charge_level: u32,
    is_disposed: bool,
}

impl Battery {
    fn use_up(mut self) {
        self.charge_level = 0;
        self.is_disposed = true;
    }

    fn dispose(mut self) {
        if self.is_disposed {
            print!("Battery already disposed...")
        } else {
            self.is_disposed = true;
        }
    }
}

#[test]
fn battery_structure_test() {
    let battery1 = Battery {
        charge_level: 50,
        is_disposed: false,
    };
    battery1.use_up();

    let battery2 = Battery {
        charge_level: 50,
        is_disposed: true,
    };
    battery2.dispose();
}
