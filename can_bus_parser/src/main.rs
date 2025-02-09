#[derive(Debug)]
struct EngineStatus {
    rpm: u8,
    coolant_temp: u8,
    oil_pressure: u8,
}

#[derive(Debug)]
struct BrakeSystem {
    brake_pressure: u8,
    abs_active: bool,
}

#[derive(Debug)]
struct Transmission {
    gear_position: u8,
    clutch_engaged: bool,
}

#[derive(Debug)]
enum CanMessage {
    Engine(EngineStatus),
    Brake(BrakeSystem),
    Trans(Transmission),
    Unknown,
}

fn parse_can_message(data: [u8; 8]) -> CanMessage {
    match data[0] {
        0x01 => CanMessage::Engine(EngineStatus {
            rpm: data[1],
            coolant_temp: data[2],
            oil_pressure: data[3],
        }),
        0x02 => CanMessage::Brake(BrakeSystem {
            brake_pressure: data[1],
            abs_active: data[2] != 0,
        }),
        0x03 => CanMessage::Trans(Transmission {
            gear_position: data[1],
            clutch_engaged: data[2] != 0,
        }),
        _ => CanMessage::Unknown,
    }
}

fn process_message(message: CanMessage) {
    match message {
        CanMessage::Engine(status) => {
            println!("Engine Status:");
            println!("  RPM: {}", status.rpm);
            println!("  Coolant Temperature: {}°C", status.coolant_temp);
            println!("  Oil Pressure: {} PSI", status.oil_pressure);
        }
        CanMessage::Brake(status) => {
            println!("Brake System:");
            println!("  Brake Pressure: {}", status.brake_pressure);
            println!("  ABS Active: {}", status.abs_active);
        }
        CanMessage::Trans(status) => {
            println!("Transmission:");
            println!("  Gear Position: {}", status.gear_position);
            println!("  Clutch Engaged: {}", status.clutch_engaged);
        }
        CanMessage::Unknown => {
            println!("Unknown Message Type");
        }
    }
    println!("------------------------");
}

fn main() {
    let messages = [
        [0x01, 0x1F, 0x40, 90, 45, 0, 0, 0], // EngineStatus
        [0x02, 80, 1, 0, 0, 0, 0, 0],        // BrakeSystem
        [0x03, 3, 1, 0, 0, 0, 0, 0],         // Transmission
        [0x04, 1, 1, 1, 1, 1, 1, 1],         // Unknown
    ];

    for msg in messages {
        let parsed_msg = parse_can_message(msg);
        process_message(parsed_msg);
    }
}
