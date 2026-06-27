use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct AtmosReading {
    battery_level: u8,
    wind_speed_kmh: f64,
    wind_direction_deg: u16,
    rainfall: f64,
    uv_index: u8,
    temperature_c: f64,
    humidity: f64,
}

impl AtmosReading {
    pub fn new() -> Self {
        Self {
            battery_level: 100,
            wind_speed_kmh: 20.5,
            wind_direction_deg: 200,
            rainfall: 3.4,
            uv_index: 4,
            temperature_c: 90.4,
            humidity: 40.0,
        }
    }

    pub fn battery_level(&self) -> u8 {
        self.battery_level
    }

    pub fn into_parts(&self) -> (u8, f64, u16, f64, u8, f64, f64) {
        let battery_level = self.battery_level;
        let wind_speed = self.wind_speed_kmh;
        let wind_direction = self.wind_direction_deg;
        let rainfall = self.rainfall;
        let uv_index = self.uv_index;
        let temperature = self.temperature_c;
        let humidity = self.humidity;
        (
            battery_level,
            wind_speed,
            wind_direction,
            rainfall,
            uv_index,
            temperature,
            humidity,
        )
    }
}
