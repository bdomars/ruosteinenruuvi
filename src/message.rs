use hwaddr::HwAddr;
use ruuvi_sensor_protocol::{SensorValues, MacAddress, Temperature, Humidity, Pressure, MeasurementSequenceNumber};

#[derive(Clone, Copy, Debug)]
pub struct RuuviMessage {
    hwaddr: HwAddr,
    temperature: f32, // temperature as C
    humidity: u8, // Relative humidity in %
    pressure: u16, // Atmospheric pressure in hPa
    seq: u32  // Message sequence number
}

impl From<SensorValues> for RuuviMessage {
    fn from(sv: SensorValues) -> Self {
        RuuviMessage{
            hwaddr: HwAddr::from(sv.mac_address().unwrap_or_default()),
            temperature: sv.temperature_as_millicelsius().unwrap_or_default() as f32 / 1000.0,
            humidity: (sv.humidity_as_ppm().unwrap_or_default() / 10_000) as u8,
            pressure: (sv.pressure_as_pascals().unwrap_or_default() / 100) as u16,
            seq: sv.measurement_sequence_number().unwrap_or_default(),
        }
    }
}

impl RuuviMessage {
    fn temperature(&self) -> f32 {
        self.temperature
    }

    fn mac(&self) -> HwAddr {
        self.hwaddr
    }

    fn humidity(&self) -> u8 {
        self.humidity
    }

    fn pressure(&self) -> u16 {
        self.pressure
    }

    fn seq(&self) -> u32 {
        self.seq
    }
}