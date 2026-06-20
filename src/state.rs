use std::sync::{Arc, RwLock};
use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]    
struct AppState{
    pub date : String,
    pub hour : String, 
    pub battery : u32,
    pub sleep : bool,

}

impl AppState {
    pub fn new() -> Arc<RwLock<AppState>> {
        let now = Local::now();
        Arc::new(RwLock::new(Self{
            date : now.format("%Y/%m/%d").to_string(),
            hour : now.format("%H:%M:%S").to_string(),
            battery : 0,
            sleep : false
        }))
    }
    pub fn update(fixed_state : &mut Arc<RwLock<AppState>>, new_state : AppState ) {
        let mut writter_guard = fixed_state.write().unwrap();
        *writter_guard = new_state;
    }
}



