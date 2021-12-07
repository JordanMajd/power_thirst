use std::error::Error;
use std::time::{ Duration};

use tokio::time;
// futures for notification stream.next
use futures::stream::StreamExt;

use btleplug::api::{Central, CharPropFlags, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Manager, Peripheral};

mod frame;
use frame::{Parse, PowerFrame};


// 0000000000100011
// PedalPowerBalancePresent
// PedalPowerBalanceReference
// CrankRevolutionDataPresent
const ASSIOMA_LEFT: &str = "ASSIOMA64394L";
const ASSIOMA_RIGHT: &str = "ASSIOMA37890R";
const KIKR_PERIPHERAL: &str = "KICKR CORE BA1B";

const TARGET_PERIPHERAL: &str = ASSIOMA_LEFT;

const SERVICE_CYCLING_SPEED_CADENCE: &str = "1816";
const SERVICE_CYCLING_POWER: &str = "1818";
const SERVICE_HEART_RATE: &str = "180D";
const SERVICE_BATTERY: &str = "180F";

const CHARACTERISTIC_POWER_MEASUREMENT: &str = "2a63";

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
        let frame = PowerFrame::parse_ble(notification.value);
        println!("{{ \"time\":\"{}\",\"watts\":\"{}\",\"balance\":\"0.{}\" }}", frame.time, frame.watts, frame.balance);
    }

    peripheral.disconnect().await.expect("Unable to disconnect");
    println!("Disconnected");

    Ok(())
}
