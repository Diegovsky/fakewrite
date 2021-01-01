use log::{log, warn, trace, debug};

pub mod prelude {
    pub use log::{log, warn, trace, debug, error};
}

pub fn init_logger(debug: bool) -> anyhow::Result<()> {
    fern::Dispatch::new()
    .format(|out, msg, record| {
        out.finish(format_args!("[{}] {}", record.level(), msg))
    })
    .level(if debug {log::LevelFilter::max()} else {log::LevelFilter::Off})
    .chain(std::io::stderr())
    .apply()?;
    Ok(())
}