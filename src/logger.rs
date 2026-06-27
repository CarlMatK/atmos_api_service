use tracing_subscriber::{self, EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub fn implement() {
    let formato = fmt::layer().with_target(false).with_ansi(true);
    let filtro = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("warn,atmos_api_service=debug"));

    tracing_subscriber::registry()
        .with(formato)
        .with(filtro)
        .init();
}
