// See the "macOS permissions note" in README.md before running this on macOS
// Big Sur or later.
use std::collections::HashMap;
use std::error::Error;
use std::time::SystemTime;

use btleplug::api::{Central, CentralEvent, Manager as _, ScanFilter};
use btleplug::platform::{Adapter, Manager};
use escape_string::escape;
use futures::stream::StreamExt;
use hwaddr::HwAddr;
use influxdb2::Client;
use ruuvi_sensor_protocol::{MacAddress, SensorValues, Temperature};

struct RuuviTag {
    name: String,
    address: HwAddr,
    sensorvalues: SensorValues,
}

async fn get_central(manager: &Manager) -> Adapter {
    let adapters = manager.adapters().await.unwrap();
    adapters.into_iter().nth(0).unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let known_sensors = HashMap::from([
        (
            HwAddr::from([0xc4, 0xb, 0x5, 0xbd, 0xe9, 0xd5]),
            String::from("Meltdown Britney"),
        ),
        (
            HwAddr::from([0xd1, 0xd8, 0x9a, 0xe1, 0x46, 0x3a]),
            String::from("Climate Change Joe"),
        ),
    ]);

    let influx_host = std::env::var("INFLUXDB_HOST").unwrap_or("http://127.0.0.1:8086".to_string());
    let influx_org = std::env::var("INFLUXDB_ORG").unwrap_or("Walkbase Office".to_string());
    let influx_token = std::env::var("INFLUXDB_TOKEN").unwrap_or(
        "PhYjnngPEOA8aUKHzJm9P5-YIkSOstJUMOp8j-zBZSTkiC7mVimp92q5x7_P3YQQ1zVoy81Rpukgo0CKtfhdXQ=="
            .to_string(),
    );

    let influx_client = Client::new(influx_host, &influx_org, influx_token);

    let health = influx_client.health().await;
    if health.is_err() {
        println!("{:#?}", health);
        panic!("no influx :(");
    }

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
                    let sensordata =
                        SensorValues::from_manufacturer_specific_data(0x0499, &ruuvidata)?;
                    let hwaddr = HwAddr::from(sensordata.mac_address().unwrap());

                    let ruuvitag = RuuviTag {
                        name: known_sensors
                            .get(&hwaddr)
                            .unwrap_or(&String::from("unknown"))
                            .to_owned(),
                        address: hwaddr,
                        sensorvalues: sensordata,
                    };

                    let temperature = (ruuvitag.sensorvalues.temperature_as_millicelsius().unwrap()
                        as f64)
                        / 1000.0;

                    let duration_since_epoch = SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap();

                    let timestamp_nanos = duration_since_epoch.as_nanos(); // u128

                    let linestr = format!(
                        "measurement,mac={},sensorname={} temp={} {}",
                        ruuvitag.address,
                        escape(&ruuvitag.name),
                        temperature,
                        timestamp_nanos,
                    );

                    let res = influx_client
                        .write_line_protocol(&influx_org, "office", linestr)
                        .await;

                    if res.is_err() {
                        println!("failed to write data: {}", res.unwrap_err());
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}
