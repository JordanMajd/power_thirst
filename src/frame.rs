use std::time::{ SystemTime, UNIX_EPOCH};

pub trait Parse<T> {
    fn parse_ble(buffer: Vec<u8>) -> T;
    // fn parse_ant(buffer: Vec<u8>) -> T;
}

pub struct PowerFrame {
    watts: i16,
    // 0 == r, 100 = left
    balance: u8,
    time: u128,
}

impl Parse<PowerFrame> for PowerFrame {
    fn parse_ble(buffer: Vec<u8>) -> PowerFrame {
        // create padded buffer
        let mut buf: [u8; 16] = [0; 16];
        for (index, val) in buffer.iter().enumerate() {
            buf[index] = *val;
        }
        let num: u128 = u128::from_le_bytes(buf);
        let watts: i16 = ((num << 96) >> 112) as i16;
        // TODO only if bitflag is set?
        let balance: u8 = (((num << 80) >> 112) as u8) / 2;
        PowerFrame {
            time: SystemTime::now().duration_since(UNIX_EPOCH).expect("Could not get time").as_millis(),
            watts: watts,
            balance: balance,
        }
    }
}



// struct PowerFrame {
//     watts: i16,
//     // 0 == r, 100 = left
//     balance: u8,
//     time: u128,
// }

// The fields in the above table, reading from top to bottom,
//     are shown in the order of LSO to MSO, where LSO = Least
//     Significant Octet and MSO = Most Significant Octet. The Least
//     Significant Octet represents the eight bits numbered 0 to 7
// enum CPS {
//     PedalPowerBalancePresent,   // bit 0
//     PedalPowerBalanceReference, // 0 = unknown, 1 = left
//     AccumulatedTorquePresent,
//     AccumulatedTorqueSource, // 0 = wheel, 1 = crank
//     WheelRevolutionDataPresent,
//     CrankRevolutionDataPresent,
//     ExtremeForceMagnitudesPresent,
//     ExtremeTorqueMagnitudesPresent,
//     ExtremeAnglesPresent,
//     TopDeadSpotAnglePresent,
//     BottomDeadSpotAnglePresent,
//     AccumulatedEnergyPresent,
//     OffsetCompensationIndicator,                   // bit 12
//     ReservationForFutureUse,                       // bit 13-15
//     InstantaneousPower,                            // sint16, mandatory bit 16-31
//     PedalPowerBalance,                             // uint8, optional
//     AccumulatedTorque,                             // uint16, optional
//     WheelRevolutionDataCumulativeWheelRevolutions, // uint32, C1
//     WheelRevolutionDataLastWheelEventTime,         // uint16, C1
//     CrankRevolutionDataCumulativeCrankRevolutions, // uint16, C2
//     CrankRevolutionDataLastCrankEventTime,         //  uint16, C2
//     ExtremeForceMagnitudesMaximumForceMagnitude,   // sint16, C3
//     ExtremeForceMagnitudesMinimumForceMagnitude,   // sint16, C3
//     ExtremeTorqueMagnitudesMaximumTorqueMagnitude, // sing16, C4
//     ExtremeTorqueMagnitudesMinimumTorqueMagnitude, // sint16, C4
//     ExtremeAnglesMaximumAngle,                     // uint12, C5
//     ExtremeAnglesMinimumAngle,                     // uint12, C5
//     TopDeadSpotAngle,                              // uint16, Optional
//     BottomDeadSpotAngle,                           // uint16, Optional
//     AccumulatedEnergy,                             // uint16, optional exponent 3
// }