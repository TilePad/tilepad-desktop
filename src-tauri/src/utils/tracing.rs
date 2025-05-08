use std::path::PathBuf;

use rolling_file::{BasicRollingFileAppender, RollingConditionBasic};
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    EnvFilter,
    fmt::{self, format::FmtSpan, writer::MakeWriterExt},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

pub fn setup_main_subscriber(logs_path: PathBuf) -> anyhow::Result<WorkerGuard> {
    // Initialize rolling file for persisted logs
    let file_appender = BasicRollingFileAppender::new(
        logs_path.join("tilepad.log"),
        RollingConditionBasic::new()
            // Roll log file daily
            .daily()
            // Max file size of 10Mb
            .max_size(1024 * 1024 * 10),
        5,
    )?;

    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let filter = EnvFilter::from_default_env();

    // Formatting for file logs
    let file_format = fmt::format()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(false)
        .with_target(false)
        .with_ansi(false)
        .compact();

    // Formatting for console logs
    let console_format = fmt::format()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(false)
        .with_target(false)
        .compact();

    // Write everything to file
    let file_layer = fmt::Layer::default()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_writer(non_blocking)
        .with_ansi(false) // explicitly disable ANSI colors for file
        .event_format(file_format);

    // Write the `ERROR` and `WARN` levels to stderr.
    let stderr_layer = fmt::Layer::default()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_writer(std::io::stderr.with_max_level(Level::WARN))
        .event_format(console_format.clone());

    // Write `INFO` to `stdout` in production
    #[cfg(not(debug_assertions))]
    let stdout_writer = std::io::stdout
        .with_max_level(Level::INFO)
        .with_min_level(Level::INFO);

    // Write all debug to `stdout` in development
    #[cfg(debug_assertions)]
    let stdout_writer = std::io::stdout
        .with_max_level(Level::DEBUG)
        .with_min_level(Level::INFO);

    let stdout_layer = fmt::Layer::default()
        .with_writer(stdout_writer)
        .event_format(console_format);

    tracing_subscriber::registry()
        .with(filter)
        .with(file_layer)
        .with(stderr_layer)
        .with(stdout_layer)
        .init();

    Ok(guard)
}

pub type PluginSubscriber = fmt::Subscriber<
    fmt::format::DefaultFields,
    fmt::format::Format<fmt::format::Compact>,
    EnvFilter,
    tracing_appender::non_blocking::NonBlocking,
>;

pub fn create_plugin_logger(logs_path: PathBuf) -> anyhow::Result<(PluginSubscriber, WorkerGuard)> {
    // Initialize rolling file for persisted logs
    let file_appender = BasicRollingFileAppender::new(
        logs_path.join("plugin.log"),
        RollingConditionBasic::new()
            // Roll log file daily
            .daily()
            // Max file size of 10Mb
            .max_size(1024 * 1024 * 10),
        5,
    )?;

    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let filter = EnvFilter::from_default_env();

    // Formatting for file logs
    let file_format = fmt::format()
        .with_file(false)
        .with_line_number(false)
        .with_thread_ids(false)
        .with_target(false)
        .with_ansi(false)
        .with_level(false)
        .compact();

    // Write everything to file
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(non_blocking)
        .with_ansi(false) // explicitly disable ANSI colors for file
        .event_format(file_format)
        .finish();

    Ok((subscriber, guard))
}
