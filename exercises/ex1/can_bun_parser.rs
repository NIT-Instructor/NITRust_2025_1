enum CanMessage {
    EngineStatus {
        rpm: u8,
        coolant_temp: u8,
        oil_pressure: u8,
    },
    BrakeSystem {
        brake_pressure: u8,
        abs_active: bool,
    },
    Transmission {
        gear_position: u8,
        clutch_engaged: bool,
    },
    Unknown,
}

fn parse_can_message(raw: [u8; 8]) -> CanMessage {
    match raw[0] {
        0x01 => CanMessage::EngineStatus {
            rpm: raw[1],
            coolant_temp: raw[2],
            oil_pressure: raw[3],
        },
        0x02 => CanMessage::BrakeSystem {
            brake_pressure: raw[1],
            abs_active: raw[2] != 0,
        },
        0x03 => CanMessage::Transmission {
            gear_position: raw[1],
            clutch_engaged: raw[2] != 0,
        },
        _ => CanMessage::Unknown,
    }
}

fn process_message(msg: CanMessage) {
    match msg {
        CanMessage::EngineStatus {
            rpm,
            coolant_temp,
            oil_pressure,
        } => {
            println!(
                "Engine Status: RPM={}, Coolant Temp={}°C, Oil Pressure={}",
                rpm, coolant_temp, oil_pressure
            );
        }
        CanMessage::BrakeSystem {
            brake_pressure,
            abs_active,
        } => {
            println!(
                "Brake System: Pressure={}, ABS Active={}",
                brake_pressure, abs_active
            );
        }
        CanMessage::Transmission {
            gear_position,
            clutch_engaged,
        } => {
            println!(
                "Transmission: Gear Position={}, Clutch Engaged={}",
                gear_position, clutch_engaged
            );
        }
        CanMessage::Unknown => {
            println!("Unknown CAN message received.");
        }
    }
}

#[test]
fn can_bun_parser_test() {
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
