use fern::InitError;
use crate::app_context::AppContext;
use crate::multi_error::MultiError;

///
/// Set up the logger
/// 
pub fn setup_logger() -> Result<(), InitError> {
    fern::Dispatch::new()
        .format(|out, message, record | {
            out.finish(format_args!(
                "{}: {}: {}: {}",
                chrono::Local::now().format("%Y%m%d.%H%M%S"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

pub fn read_file(app_ctx: &mut AppContext) -> Result<(), MultiError> {
    let mut file = File::open(app_ctx.input_file)
}

pub fn read_byte(app_ctx: &AppContext) -> u8 {

}