// See the "macOS permissions note" in README.md before running this on macOS
// Big Sur or later.

use btleplug::api::{Central, CentralEvent, Manager as _, ScanFilter};
use btleplug::platform::{Adapter, Manager};
use futures::stream::StreamExt;
use std::error::Error;
use ruuvi_sensor_protocol::{SensorValues, Temperature, MacAddress, MeasurementSequenceNumber};
use std::fmt::Display;
use std::collections::HashMap;
use hwaddr::HwAddr;

struct RuuviTag {
    name: String,
    address: HwAddr,
    sensorvalues: SensorValues,
}

impl Display for RuuviTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let temperature = (self.sensorvalues.temperature_as_millicelsius().unwrap() as f64) / 1000.0;
        let seqnr = self.sensorvalues.measurement_sequence_number().unwrap_or(0);
        write!(f, "{} ({})\tm#{}\t{}°C", self.name, self.address, seqnr, temperature)?;
        Ok(())
    }
}

async fn get_central(manager: &Manager) -> Adapter {
    let adapters = manager.adapters().await.unwrap();
    adapters.into_iter().nth(0).unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let known_sensors = HashMap::from([
        (HwAddr::from([0xc4, 0xb, 0x5, 0xbd, 0xe9, 0xd5]), String::from("Meltdown Britney")),
        (HwAddr::from([0xd1, 0xd8, 0x9a, 0xe1, 0x46, 0x3a]), String::from("Climate Change Joe")),
    ]);


    let manager = Manager::new().await?;

    // get the first bluetooth adapter
    // connect to the adapter
    let central = get_central(&manager).await;

    // Each adapter has an event stream, we fetch via events(),
    // simplifying the type, this will return what is essentially a
    // Future<Result<Stream<Item=CentralEvent>>>.
    let mut events = central.events().await?;

    // start scanning for devices
    central.start_scan(ScanFilter::default()).await?;

    // Print based on whatever the event receiver outputs. Note that the event
    // receiver blocks, so in a real program, this should be run in its own
    // thread (not task, as this library does not yet use async channels).
    while let Some(event) = events.next().await {
        match event {
            CentralEvent::ManufacturerDataAdvertisement {
                id: _,
                manufacturer_data,
            } => {
                if let Some(ruuvidata) = &manufacturer_data.get(&0x0499) {
                    let sensordata = SensorValues::from_manufacturer_specific_data(0x0499, &ruuvidata)?;
                    let hwaddr = HwAddr::from(sensordata.mac_address().unwrap());

                    let ruuvitag = RuuviTag{
                        name: known_sensors.get(&hwaddr).unwrap_or(&String::from("unknown")).to_owned(),
                        address: hwaddr,
                        sensorvalues: sensordata,
                    };

                

                    println!("{}", ruuvitag)
                }
            }
            _ => {}
        }
    }
    Ok(())
}