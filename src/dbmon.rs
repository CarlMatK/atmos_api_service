use turso::{Builder, Connection, Rows};

use crate::types::AtmosData;
use tuple_join::{self, Join};

#[derive(Clone)]
pub struct TursoDB {
    conn: Connection,
}

impl TursoDB {
    pub async fn new() -> Self {
        let db = Builder::new_local("atmos.db")
            .build()
            .await
            .expect("No se pudo conectar al db ");
        let _conn = db.connect().unwrap();
        Self { conn: _conn }
    }
    pub async fn push_from_param(&self, param: AtmosData, str1: String, str2: String) {
        let mut stmt = self.conn.prepare("INSERT INTO reads 
        (date, time, battery_level, wind_speed_kmh, wind_direction_deg, rainfall, uv_index, temperature_c, humidity)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ,?7, ?8, ?9)").await.expect("Error al preparar el query");
        let strs = (str1, str2);
        let missing = param.into_parts();
        let fixed = strs.join(missing);

        stmt.execute(fixed)
            .await
            .expect("Error al pasarle los parametros");
    }

    pub async fn query_all(&self) -> String {
        let mut tabla = self.conn.query("SELECT * FROM reads", ()).await.unwrap();

        let renglon = tabla.next().await.unwrap().unwrap();
        println!("starting a lot of info");

        let total_data = String::new();

        println!(
            "El query arrojo una tabla de {} columnas y {:?}",
            tabla.column_count(),
            TursoDB::rows_count(&mut tabla).await
        );

        // //        for i in 0..=tabla.column_count() {

        //           let tmp1 = &renglon.get_value(i).unwrap();

        //            let tmp2 = tmp1.as_text().unwrap();
        //total_data.push_str(tmp2.clone().as_str());
        //            println!("Row : {:?}", tmp2);

        //        }
        total_data
    }

    pub async fn rows_count(tabla: &mut Rows) -> u128 {
        if tabla.next().await.unwrap().is_none() {
            return 0;
        }
        let mut renglones = 1;
        loop {
            match tabla.next().await {
                Ok(a) => {
                    if a.is_none() {
                        break;
                    }
                    println!("sumandose 1 a la cantidad de renglones: {}", renglones);
                    renglones += 1;
                }
                Err(_) => {
                    break;
                }
            }
        }
        renglones
    }

    pub async fn query_by_date(&mut self, date: &str) -> String {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM reads WHERE date = ?1")
            .await
            .unwrap();
        let mut rows = stmt.query([date]).await.unwrap();
        let row = rows.next().await.unwrap().unwrap();
        let val = row.get_value(0).unwrap();
        println!("Value selected row : {:?}", &val);

        format!("Row: {:?}", val)
    }
}
