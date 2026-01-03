use configuration::{LoggingFormat, LoggingLevel, LoggingSettings};
use tracing::Subscriber;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

pub fn init_tracing(logging_settings: &LoggingSettings) {
    let env_filter = EnvFilter::builder()
        .with_default_directive(level_filter(&logging_settings.level).into())
        .from_env_lossy();

    let subscriber_builder = tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(true)
        .with_env_filter(env_filter);

    match logging_settings.format {
        LoggingFormat::Compact => register_subscriber(subscriber_builder.compact().finish()),
        LoggingFormat::Json => register_subscriber(subscriber_builder.json().finish()),
        LoggingFormat::Pretty => register_subscriber(subscriber_builder.pretty().finish()),
        LoggingFormat::Full => register_subscriber(subscriber_builder.finish()),
    }
}

fn register_subscriber<S>(subscriber: S)
where
    S: Subscriber + Send + Sync + 'static,
{
    tracing::subscriber::set_global_default(subscriber)
        .map_err(|_err| eprintln!("Unable to set global default subscriber"))
        .unwrap();
}

fn level_filter(logging_level: &LoggingLevel) -> LevelFilter {
    match logging_level {
        LoggingLevel::Debug => LevelFilter::DEBUG,
        LoggingLevel::Info => LevelFilter::INFO,
        LoggingLevel::Warn => LevelFilter::WARN,
        LoggingLevel::Error => LevelFilter::ERROR,
        LoggingLevel::Trace => LevelFilter::TRACE,
    }
}
