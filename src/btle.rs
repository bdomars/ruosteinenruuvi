use btleplug::api::{Central, CentralEvent, Manager as _, ScanFilter};
use btleplug::platform::{Adapter, Manager};
use futures::stream::StreamExt;
use ruuvi_sensor_protocol::SensorValues;
use tokio::sync::broadcast::Sender;

use crate::RuuviMessage;

const RUUVI: u16 = 0x499;

async fn get_central(manager: &Manager) -> Adapter {
    let adapters = manager.adapters().await.unwrap();
    adapters.into_iter().nth(0).unwrap()
}

pub async fn scan_btle(tx: Sender<RuuviMessage>) -> crate::Result<()> {
    let manager = Manager::new().await?;
    let central = get_central(&manager).await;
    let mut events = central.events().await?;
    central.start_scan(ScanFilter::default()).await?;
    while let Some(event) = events.next().await {
        match event {
            CentralEvent::ManufacturerDataAdvertisement {
                id: _,
                manufacturer_data,
            } => {
                if let Some(md) = manufacturer_data.get(&RUUVI) {
                    let sv = match SensorValues::from_manufacturer_specific_data(RUUVI, md) {
                        Ok(sv) => sv,
                        Err(err) => {
                            println!("could not decode sensor data: {}", err);
                            continue
                        }
                    };
                    let rm = RuuviMessage::from(sv);
                    tx.send(rm)?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}
