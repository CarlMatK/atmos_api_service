use crate::dbmon::TursoDB;
use chrono::Local;
use std::sync::{Arc, RwLock};

impl TimeStamp {
    pub fn new() -> Self {
        let now = Local::now();
        Self {
            date: now.format("%Y-%m-%d").to_string(),
            time: now.format("%H:%M:%S").to_string(),
        }
    }
    fn rerun(&mut self) {
        let now = Local::now();
        self.date = now.format("%Y-%m-%d").to_string();
        self.time = now.format("%H:%M:%S").to_string();
    }
}

impl AppState {
    pub async fn new() -> Arc<AppState> {
        Arc::new(Self {
            pool: TursoDB::new().await,
            station: Arc::new(RwLock::new(Status {
                battery_lvl: 100,
                last_stamp: TimeStamp::new(),
            })),
        })
    }
    pub fn stamp_intoparts(&self) -> (String, String) {
        let lock_clone = Arc::clone(&self.station);
        let reader = lock_clone.read().unwrap();
        let s1 = reader.last_stamp.date.clone();
        let s2 = reader.last_stamp.time.clone();
        (s1, s2)
    }
    pub fn rerun_stamp(&self) {
        let mut writer = self.station.write().unwrap();
        writer.last_stamp.rerun();
    }
    pub fn upd_battery(&self, batt: u8) {
        let mut writer = self.station.write().unwrap();
        writer.battery_lvl = batt;
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
    last_stamp: TimeStamp,
}

#[derive(Clone)]
struct TimeStamp {
    date: String,
    time: String,
}
