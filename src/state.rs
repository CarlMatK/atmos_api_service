use std::sync::{Arc, RwLock};
use chrono::Local;
use dotenvy::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::result::Error;



#[derive(Clone)]
struct LastRead{
    hour : Arc<RwLock<String>>, 
    date : Arc<RwLock<String>>,
    timestamp : Arc<RwLock<String>>
}

#[derive(Clone)]
struct BatteryStatus{
    amps_h : Arc<RwLock<u32>>    
}

#[derive(Clone)]
struct SqlitePool{
    //pool : Pool<ConnectionManager<DbConnection>>
}

// impl SqliteConn {
//     pub fn from(val: Arc<RwLock<SqliteConnection>>) -> Self {
//         Self { conn:  }
//     }
// }
impl BatteryStatus {
    pub fn from(val : u32) -> Self{
        Self { amps_h: Arc::new(RwLock::new(val))}
    }
    pub fn from_fixed(val: Arc<RwLock<u32>>) -> Self {
        Self { amps_h: val }
    }
}
impl LastRead {
    pub fn new() -> Self {
        let now = Local::now();
        let date_f = Arc::new(RwLock::new(now.format("%Y-%m-%d").to_string()));
        let hour_f = Arc::new(RwLock::new(now.format("%H/%M/%S").to_string()));
        let timestamp_f = Arc::new(RwLock::new(now.format("%Y-%m-%d %H/%M/%S").to_string()));
        Self { 
            date : date_f,
            hour : hour_f,
            timestamp : timestamp_f
        }
    }
}

#[derive(Clone)]    
struct AppState{
    pub battery_status : BatteryStatus,
    pub last_read : LastRead,
    //pub sqlite_conn : SqliteConn,
}

impl AppState {
    fn new() -> Self {
        dotenv().ok();
        
        // ruta de la db dentro de .env 
        let database_url = env::var("DATABASE_URL").expect("Falta la variable DATABASE_URL");
        // conexion a db
        let sqlite_conn = SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error al conectarse a {}", database_url));
        // valor encapsulado 
        let fixed_conn = Arc::new(RwLock::new(sqlite_conn));
        
        Self{
            battery_status : BatteryStatus::from(0),
            last_read : LastRead::new(),
            //sqlite_conn : SqliteConn::from(fixed_conn)
        }
    }
}