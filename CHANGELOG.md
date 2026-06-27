# Changelog

Todos los cambios importantes de este proyecto se documentaran en este archivo.

El formato esta basado en [Keep a Changelog](https://keepachangelog.com/es-ES/1.1.0/)
y este proyecto sigue [Semantic Versioning](https://semver.org/lang/es/).

## [Unreleased]

## [0.3.0] - 2026-06-26

### Added
- Se agrego un modulo `logger` para inicializar la observabilidad del servicio.
- Se incorporo configuracion base de `tracing` y `tracing-subscriber` para su arranque en la API.
- Se agrego instrumentacion con `#[instrument]` en handlers y operaciones clave de base de datos.
- Se añadio contexto de spans en `upload_data` para exponer `battery_level` sin registrar el payload completo.

### Changed
- Se reemplazaron mensajes sueltos y `println!` por eventos estructurados con `tracing::info!` y `tracing::debug!`.
- El filtro por defecto del logger local ahora usa `warn,atmos_api_service=debug` para priorizar detalle del crate sin habilitar `trace`.
- El arranque del servicio, la construccion del router y el flujo principal de consultas e inserciones ahora reportan 'hitos' operativos via logging estructurado.
- Se quito del gitignore el atmos.db para no tener errores al momento de ejecutar los pipeline 
- Se ejecuto cargo fmt --all -- --check para validar que el formato contraste con el que el pipeline require

### Fixed
- Se redujo el ruido de salida en la capa de base de datos al mover diagnosticos a `debug` y dejarlos bajo el control de `RUST_LOG`.

## [0.2.0] - 2026-06-25

### Added
- API HTTP con `axum`.
- Endpoint `GET /v01/data`.
- Endpoint `POST /v01/data`.
- Integracion inicial con Turso.
- Modelo `AtmosData`.
- Pruebas iniciales del servicio.

### Changed
- La descarga devuelve un maximo de 10 registros.

### Fixed
- Ajustes a tests y pipeline de integracion.
