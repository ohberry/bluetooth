use btleplug::api::{bleuuid::uuid_from_u16, Central, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::{Adapter, Manager, Peripheral};
use rand::{Rng, thread_rng};
use std::error::Error;
use std::thread;
use std::time::Duration;
use tokio::time;
use uuid::Uuid;


// const LIGHT_CHARACTERISTIC_UUID: Uuid = uuid_from_u16(0xFFE9);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await.unwrap();

    // get the first bluetooth adapter
    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().nth(0).unwrap();

    // start scanning for devices
    central.start_scan(ScanFilter::default()).await?;
    // instead of waiting, you can use central.events() to get a stream which will
    // notify you of new devices, for an example of that see examples/event_driven_discovery.rs
    time::sleep(Duration::from_secs(2)).await;

    // print_all_names(&central).await;

    // find the device we're interested in
    let light: Peripheral = find_light(&central).await.unwrap();

    // connect to the device
    light.connect().await?;

    // discover services and characteristics
    light.discover_services().await?;

    // find the characteristic we want
    let chars = light.characteristics();
    for elem in &chars {
        println!("{:?}", elem.uuid);
    }
    let uid = Uuid::parse_str("00002a00-0000-1000-8000-00805f9b34fb").unwrap();
    let cmd_char = chars.iter().find(|c| c.uuid == uid).unwrap();

    // dance party
    // let mut rng = thread_rng();
    // for _ in 0..20 {
    //     let color_cmd = vec![0x56, rng.gen(), rng.gen(), rng.gen(), 0x00, 0xF0, 0xAA];
    //     light.write(&cmd_char, &color_cmd, WriteType::WithoutResponse).await?;
    //     time::sleep(Duration::from_millis(200)).await;
    // }
    // let result = light.read(&cmd_char).await?;
    // println!("{:?}", result);
    light.subscribe(&cmd_char).await?;
    let mut event_stream = light.notifications().await?;
    while let Some(data) = event_stream.().await {
        
    }

    Ok(())
}

async fn find_light(central: &Adapter) -> Option<Peripheral> {
    for p in central.peripherals().await.unwrap() {
        if p.properties()
            .await
            .unwrap()
            .unwrap()
            .local_name
            .iter()
            .any(|name| name.contains("HUAWEI Band HR-C8A"))
        {
            return Some(p);
        }
    }
    None
}

async fn print_all_names(central: &Adapter) {
    for p in central.peripherals().await.unwrap() {
        if let Ok(properties) = p.properties().await {
            if let Some(local_name) = properties.unwrap().local_name {
                println!("Device Name: {}", local_name);
            } else {
                println!("Device has no name.");
            }
        } else {
            println!("Failed to get properties for a device.");
        }
    }
}