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
        let mut stmt = self.conn.prepare("INSERT INTO reads (date, time, battery_level, wind_speed_kmh, wind_direction_deg, rainfall, uv_index, temperature_c, humidity) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9);").await.expect("Error al preparar el query");
        let strs: (String, String) = (str1, str2);
        let missing: (u8, f64, u16, f64, u8, f64, f64) = param.into_parts();
        let fixed: (String, String, u8, f64, u16, f64, u8, f64, f64) = strs.join(missing);

        stmt.execute(fixed)
            .await
            .expect("Error al pasarle los parametros");
    }

    pub async fn query_all(&self) -> String {
        let mut tabla = self.conn.query("SELECT * FROM reads", ()).await.unwrap();

        println!("starting a lot of info");
        println!(
            "El query arrojo una tabla de {} columnas y ",
            tabla.column_count(),
        );

        //let renglones = TursoDB::rows_count(&mut tabla).await;
        let table_str = TursoDB::get_from_sized(10, &mut tabla).await;

        match table_str {
            Some(n) => {
                println!(" data was: {}", &n);
                n
            }
            _ => String::from("Err while getting values"),
        }
    }

    pub async fn rows_count(tabla: &mut Rows) -> usize {
        if None == tabla.next().await.unwrap() {
            println!("la tabla esta vacia?");
            return 2;
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

    pub async fn get_from_sized(rows: usize, table: &mut Rows) -> Option<String> {
        let mut table_info = String::from("date, time, batt, wkmh, wdeg, rain, uvdx, temp, humi");
        println!("size: {}",&rows);
        for c in 0..rows {
            let txt = format!("\n{}: ", c.clone());
            table_info.push_str(txt.as_str());

            if let Some(renglon) = table.next().await.ok()? {
                let v0 = renglon.get_value(0).ok()?;
                let v1 = renglon.get_value(1).ok()?;
                let v2 = renglon.get_value(2).ok()?;
                let v3 = renglon.get_value(3).ok()?;
                let v4 = renglon.get_value(4).ok()?;
                let v5 = renglon.get_value(5).ok()?;
                let v6 = renglon.get_value(6).ok()?;
                let v7 = renglon.get_value(7).ok()?;
                let v8 = renglon.get_value(8).ok()?;

                let date = v0.as_text()?;
                let time = v1.as_text()?;
                let batt = v2.as_integer()?;
                let wkmh = v3.as_real()?;
                let wdeg = v4.as_integer()?;
                let rain = v5.as_real()?;
                let uvdx = v6.as_real()?;
                let temp = v7.as_real()?;
                let humi = v8.as_integer()?;
                table_info.push_str(
                    format!(
                        "{}, {}, {}, {}, {}, {}, {}, {}, {}",
                        date, time, batt, wkmh, wdeg, rain, uvdx, temp, humi
                    )
                    .as_str(),
                );
            } else {
                break;
            }
        }
        Some(table_info)
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
