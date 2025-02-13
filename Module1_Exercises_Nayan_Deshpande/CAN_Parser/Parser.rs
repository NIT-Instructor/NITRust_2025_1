
enum CanMessage {
    EngineStatus { rpm: u8, coolant_temp: u8, oil_pressure: u8 },
    BrakeSystem { brake_pressure: u8, abs_active: bool },
    Transmission { gear_position: u8, clutch_engaged: bool },
    Unknown,  // For unknown message IDs
}

fn parse_can_message(msg: &[u8; 8]) -> CanMessage {
    match &msg[0] {
        0x01 => CanMessage::EngineStatus {
            rpm: msg[1],
            coolant_temp: msg[2],
            oil_pressure: msg[3],
        },
        0x02 => CanMessage::BrakeSystem {
            brake_pressure: msg[1],
            abs_active: msg[2] != 0,  // Assuming 0 is inactive and non-zero is active
        },
        0x03 => CanMessage::Transmission {
            gear_position: msg[1],
            clutch_engaged: msg[2] != 0,  // Assuming 0 is disengaged and non-zero is engaged
        },
        _ => CanMessage::Unknown,  // Handle unknown message IDs
    }
}

fn process_message(msg: CanMessage) {
    match msg {
        CanMessage::EngineStatus { rpm, coolant_temp, oil_pressure } => {
            println!("Engine Status - RPM: {}, Coolant Temp: {}°C, Oil Pressure: {} PSI", rpm, coolant_temp, oil_pressure);
        }
        CanMessage::BrakeSystem { brake_pressure, abs_active } => {
            println!("Brake System - Brake Pressure: {} bar, ABS Active: {}", brake_pressure, if abs_active { "Yes" } else { "No" });
        }
        CanMessage::Transmission { gear_position, clutch_engaged } => {
            println!("Transmission - Gear Position: {}, Clutch Engaged: {}", gear_position, if clutch_engaged { "Yes" } else { "No" });
        }
        CanMessage::Unknown => {
            println!("Unknown CAN message received.");
        }
    }
}

fn main() {
    // Simulate receiving multiple CAN messages in a loop
    let messages = [
        [0x01, 0x1F, 0x40, 90, 45, 0, 0, 0], // EngineStatus
        [0x02, 80, 1, 0, 0, 0, 0, 0],       // BrakeSystem
        [0x03, 3, 1, 0, 0, 0, 0, 0],        // Transmission
        [0x04, 1, 1, 1, 1, 1, 1, 1],         // Unknown
    ];

    for msg in messages.iter() {
     let parsed_msg = parse_can_message(msg);
     process_message(parsed_msg);
    }
}