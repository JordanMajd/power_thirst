use std::error::Error;
use std::time::{ Duration, SystemTime};
use tokio::time;

// futures for notification stream.next
use futures::stream::StreamExt;

use btleplug::api::{Central, CharPropFlags, CentralEvent, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Manager, Peripheral};

// 0000000000100011
// PedalPowerBalancePresent
// PedalPowerBalanceReference
// CrankRevolutionDataPresent
const ASSIOMA_LEFT: &str = "ASSIOMA64394L";
const KIKR_PERIPHERAL: &str = "KICKR CORE BA1B";

const TARGET_PERIPHERAL: &str = ASSIOMA_LEFT;

const SERVICE_CYCLING_SPEED_CADENCE: &str = "1816";
const SERVICE_CYCLING_POWER: &str = "1818";
const SERVICE_HEART_RATE: &str = "180D";
const SERVICE_BATTERY: &str = "180F";

const CHARACTERISTIC_POWER_MEASUREMENT: &str = "2a63";

struct PowerFrame {
    watts: i16,
    balance: u8,
}

// The fields in the above table, reading from top to bottom,
//     are shown in the order of LSO to MSO, where LSO = Least
//     Significant Octet and MSO = Most Significant Octet. The Least
//     Significant Octet represents the eight bits numbered 0 to 7
enum CPS {
    PedalPowerBalancePresent,   // bit 0
    PedalPowerBalanceReference, // 0 = unknown, 1 = left
    AccumulatedTorquePresent,
    AccumulatedTorqueSource, // 0 = wheel, 1 = crank
    WheelRevolutionDataPresent,
    CrankRevolutionDataPresent,
    ExtremeForceMagnitudesPresent,
    ExtremeTorqueMagnitudesPresent,
    ExtremeAnglesPresent,
    TopDeadSpotAnglePresent,
    BottomDeadSpotAnglePresent,
    AccumulatedEnergyPresent,
    OffsetCompensationIndicator,                   // bit 12
    ReservationForFutureUse,                       // bit 13-15
    InstantaneousPower,                            // sint16, mandatory bit 16-31
    PedalPowerBalance,                             // uint8, optional
    AccumulatedTorque,                             // uint16, optional
    WheelRevolutionDataCumulativeWheelRevolutions, // uint32, C1
    WheelRevolutionDataLastWheelEventTime,         // uint16, C1
    CrankRevolutionDataCumulativeCrankRevolutions, // uint16, C2
    CrankRevolutionDataLastCrankEventTime,         //  uint16, C2
    ExtremeForceMagnitudesMaximumForceMagnitude,   // sint16, C3
    ExtremeForceMagnitudesMinimumForceMagnitude,   // sint16, C3
    ExtremeTorqueMagnitudesMaximumTorqueMagnitude, // sing16, C4
    ExtremeTorqueMagnitudesMinimumTorqueMagnitude, // sint16, C4
    ExtremeAnglesMaximumAngle,                     // uint12, C5
    ExtremeAnglesMinimumAngle,                     // uint12, C5
    TopDeadSpotAngle,                              // uint16, Optional
    BottomDeadSpotAngle,                           // uint16, Optional
    AccumulatedEnergy,                             // uint16, optional exponent 3
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await?;

    // get primary adapter
    let adapter = manager
        .adapters()
        .await
        .expect("Unable to get adapter")
        .into_iter()
        .nth(0)
        .expect("Unable to get adapter");

    println!("Using adapter: {}", adapter.adapter_info().await?);
    
    
    // let mut events = adapter.events().await?;
    
    adapter
    .start_scan(ScanFilter::default())
    .await
    .expect("Cannot scan adapter");
    time::sleep(Duration::from_secs(10)).await;
    
    //     while let Some(event) = events.next().await {
    //         match event {
    //             CentralEvent::DeviceDiscovered(id) => {
    //                 println!("{:?}", id);
    //             }
    //             _ => {}
    //         };
    //     }

    let peripherals = adapter.peripherals().await?;
    for peripheral in peripherals.iter() {
        let props = peripheral.properties().await?.unwrap();
        let name = props.local_name.unwrap_or(String::from("unknown"));
        println!("Discovered: {}", name);
        if name == TARGET_PERIPHERAL {
            connect_peripheral(peripheral).await?;
        }
    }

    Ok(())
}

async fn connect_peripheral(peripheral: &Peripheral) -> Result<(), Box<dyn Error>> {
    let props = peripheral.properties().await?.unwrap();
    let name = props.local_name.unwrap_or(String::from("unknown"));
    let rssi = props.rssi.unwrap_or(0);
    println!(
        "Connecting to peripheral {} @ {}, {} rssi",
        name, props.address, rssi
    );

    let is_connected = peripheral.is_connected().await?;
    if !is_connected {
        println!("Connecting...");
        if let Err(err) = peripheral.connect().await {
            eprintln!("{}", err);
        }
    }
    let is_connected = peripheral.is_connected().await?;
    if !is_connected {
        return Ok(());
    }
    println!("Connected");

    peripheral.discover_services().await?;
    for service in peripheral.services() {
        if service.uuid.to_string().contains(SERVICE_CYCLING_POWER) {
            for characteristic in service.characteristics {
                // TODO caseing
                if characteristic
                    .uuid
                    .to_string()
                    .contains(CHARACTERISTIC_POWER_MEASUREMENT)
                    && characteristic.properties.contains(CharPropFlags::NOTIFY)
                {
                    println!(
                        "Subscribing Power Measurement characteristic {} of service {}, props {:?}",
                        characteristic.uuid, service.uuid, characteristic.properties
                    );
                    peripheral.subscribe(&characteristic).await?;
                }
            }
        }
    }

    let mut notification_stream = peripheral.notifications().await?;

    while let Some(notification) = notification_stream.next().await {
        // println!("uuid {:?}, value {:?}", notification.uuid, notification.value);
        parse_power_frame(notification.value);
    }

    peripheral.disconnect().await.expect("Unable to disconnect");
    println!("Disconnected");

    Ok(())
}

fn parse_power_frame(value: Vec<u8>) -> PowerFrame {
    // create padded buffer
    let mut buf: [u8; 16] = [0; 16];
    for (index, val) in value.iter().enumerate() {
        buf[index] = *val;
    }
    let num: u128 = u128::from_le_bytes(buf);
    let watts: i16 = ((num << 96) >> 112) as i16;
    // TODO only if bitflag is set?
    let balance: u8 = (((num << 80) >> 112) as u8) / 2;
    println!("{{ watts:\"{}\", balance: \"0.{}\" }}", watts, balance);
    PowerFrame {
        time: SystemTime::now(),
        watts: watts,
        balance: balance,
    }
}
