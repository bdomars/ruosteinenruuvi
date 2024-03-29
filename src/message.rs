#![allow(dead_code)]
use macaddr::MacAddr6;
use ruuvi_sensor_protocol::{
    Humidity, MacAddress, MeasurementSequenceNumber, Pressure, SensorValues, Temperature,
};

#[derive(Clone, Copy, Debug)]
pub struct RuuviMessage {
    hwaddr: MacAddr6,
    temperature: f32, // temperature as C
    humidity: u8,     // Relative humidity in %
    pressure: u16,    // Atmospheric pressure in hPa
    seq: u32,         // Message sequence number
}

impl From<SensorValues> for RuuviMessage {
    fn from(sv: SensorValues) -> Self {
        RuuviMessage {
            hwaddr: MacAddr6::from(sv.mac_address().unwrap_or_default()),
            temperature: sv.temperature_as_millicelsius().unwrap_or_default() as f32 / 1000.0,
            humidity: (sv.humidity_as_ppm().unwrap_or_default() / 10_000) as u8,
            pressure: (sv.pressure_as_pascals().unwrap_or_default() / 100) as u16,
            seq: sv.measurement_sequence_number().unwrap_or_default(),
        }
    }
}

impl RuuviMessage {
    pub fn temperature(&self) -> f32 {
        self.temperature
    }

    pub fn mac(&self) -> MacAddr6 {
        self.hwaddr
    }

    pub fn humidity(&self) -> u8 {
        self.humidity
    }

    pub fn pressure(&self) -> u16 {
        self.pressure
    }

    pub fn seq(&self) -> u32 {
        self.seq
    }
}
