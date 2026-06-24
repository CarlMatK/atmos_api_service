use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct AtmosData {
    battery_level: u8,
    wind_speed_kmh: f64,
    wind_direction_deg: u16,
    rainfall: f64,
    uv_index: u8,
    temperature_c: f64,
    humidity: f64,
}

impl AtmosData {
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
    pub fn into_batt(&self) -> u8 {
        self.battery_level.clone()
    }
    pub fn into_parts(&self) -> (u8, f64, u16, f64, u8, f64, f64) {
        let battlvl = self.battery_level.clone();
        let windspeed = self.wind_speed_kmh.clone();
        let winddir = self.wind_direction_deg.clone();
        let rainmm = self.rainfall.clone();
        let uv_index = self.uv_index.clone();
        let tempc = self.temperature_c.clone();
        let humper = self.humidity.clone();
        (battlvl, windspeed, winddir, rainmm, uv_index, tempc, humper)
    }
}
