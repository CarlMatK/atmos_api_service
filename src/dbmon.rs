use turso::{Builder, Connection, Rows};

use crate::types::AtmosReading;
use tuple_join::{self, Join};
use tracing::instrument;

#[derive(Clone)]
pub struct TursoDB {
    conn: Connection,
}

impl TursoDB {
    pub async fn new() -> Self {
        tracing::info!("Inicializando conexion local con Turso");
        let db = Builder::new_local("atmos.db")
            .build()
            .await
            .expect("No se pudo conectar al db ");
        let _conn = db.connect().unwrap();
        tracing::info!("Conexion con Turso lista");
        Self { conn: _conn }
    }

    #[instrument(skip(self, reading), fields(date = %date, time = %time))]
    pub async fn insert_from_param(&self, reading: AtmosReading, date: String, time: String) {
        tracing::debug!(date = %date, time = %time, "Preparando insercion de lectura en base de datos");
        let mut stmt = self.conn.prepare("INSERT INTO reads (date, time, battery_level, wind_speed_kmh, wind_direction_deg, rainfall, uv_index, temperature_c, humidity) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9);").await.expect("Error al preparar el query");
        let timestamp_parts: (String, String) = (date, time);
        let reading_values: (u8, f64, u16, f64, u8, f64, f64) = reading.into_parts();
        let query_params: (String, String, u8, f64, u16, f64, u8, f64, f64) = timestamp_parts.join(reading_values);

        stmt.execute(query_params)
            .await
            .expect("Error al pasarle los parametros");
        tracing::info!("Insercion completada en la tabla reads");
    }

    #[instrument(skip(self))]
    pub async fn fetch_all_readings(&self) -> String {
        tracing::debug!("Ejecutando consulta completa sobre la tabla reads");
        let mut table = self.conn.query("SELECT * FROM reads", ()).await.unwrap();

        let table_str = TursoDB::format_first_n_rows(10, &mut table).await;

        match table_str {
            Some(n) => {
                tracing::info!("Consulta completada; se devolvieron hasta 10 registros");
                n
            }
            _ => {
                tracing::debug!("La consulta no devolvio registros serializables");
                String::from("Err while getting values")
            }
        }
    }

    pub async fn count_rows(table: &mut Rows) -> usize {
        if None == table.next().await.unwrap() {
            tracing::debug!("La tabla consultada no contiene renglones");
            return 2;
        }
        let mut row_count = 1;
        loop {
            match table.next().await {
                Ok(a) => {
                    if a.is_none() {
                        break;
                    }
                    row_count += 1;
                }
                Err(_) => {
                    break;
                }
            }
        }
        tracing::debug!(rows = row_count, "Conteo de renglones completado");
        row_count
    }

    pub async fn format_first_n_rows(rows: usize, table: &mut Rows) -> Option<String> {
        let mut table_info = String::from("date, time, batt, wkmh, wdeg, rain, uvdx, temp, humi");
        for c in 0..rows {
            let txt = format!("\n{}: ", c);
            table_info.push_str(txt.as_str());

            if let Some(row) = table.next().await.ok()? {
                let v0 = row.get_value(0).ok()?;
                let v1 = row.get_value(1).ok()?;
                let v2 = row.get_value(2).ok()?;
                let v3 = row.get_value(3).ok()?;
                let v4 = row.get_value(4).ok()?;
                let v5 = row.get_value(5).ok()?;
                let v6 = row.get_value(6).ok()?;
                let v7 = row.get_value(7).ok()?;
                let v8 = row.get_value(8).ok()?;

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

    #[instrument(skip(self))]
    pub async fn query_by_date(&mut self, date: &str) -> String {
        tracing::debug!(date = %date, "Ejecutando consulta filtrada por fecha");
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM reads WHERE date = ?1")
            .await
            .unwrap();
        let mut rows = stmt.query([date]).await.unwrap();
        let row = rows.next().await.unwrap().unwrap();
        let val = row.get_value(0).unwrap();
        tracing::debug!(value = ?val, "Primer valor recuperado para la fecha consultada");

        format!("Row: {:?}", val)
    }
}
