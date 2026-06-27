use crate::dbmon::TursoDB;
use chrono::Local;
use std::sync::{Arc, RwLock};

impl Timestamp {
    pub fn new() -> Self {
        let now = Local::now();
        Self {
            date: now.format("%Y-%m-%d").to_string(),
            time: now.format("%H:%M:%S").to_string(),
        }
    }

    fn refresh_time(&mut self) {
        let now = Local::now();
        self.date = now.format("%Y-%m-%d").to_string();
        self.time = now.format("%H:%M:%S").to_string();
    }
}

impl AppState {
    pub async fn new() -> Arc<AppState> {
        tracing::info!("Inicializando estado compartido de la aplicacion");
        Arc::new(Self {
            pool: TursoDB::new().await,
            station: Arc::new(RwLock::new(Status {
                battery_lvl: 100,
                last_stamp: Timestamp::new(),
            })),
        })
    }

    pub fn timestamp_parts(&self) -> (String, String) {
        let lock_clone = Arc::clone(&self.station);
        let reader = lock_clone.read().unwrap();
        let date = reader.last_stamp.date.clone();
        let time = reader.last_stamp.time.clone();
        tracing::debug!(date = %date, time = %time, "Timestamp actual del estado");
        (date, time)
    }

    pub fn refresh_time(&self) {
        let mut writer = self.station.write().unwrap();
        writer.last_stamp.refresh_time();
        tracing::debug!(
            date = %writer.last_stamp.date,
            time = %writer.last_stamp.time,
            "Timestamp del estado actualizado"
        );
    }

    pub fn update_battery(&self, batt: u8) {
        let mut writer = self.station.write().unwrap();
        writer.battery_lvl = batt;
        tracing::debug!(
            battery_level = batt,
            "Nivel de bateria almacenado en el estado"
        );
    }
}

#[derive(Clone)]
pub struct AppState {
    pub pool: TursoDB,
    pub station: Arc<RwLock<Status>>,
}

#[derive(Clone)]
pub struct Status {
    battery_lvl: u8,
    last_stamp: Timestamp,
}

#[derive(Clone)]
struct Timestamp {
    date: String,
    time: String,
}
