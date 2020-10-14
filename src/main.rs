pub mod util;
pub mod exe_format;
pub mod multi_error;
pub mod app_context;
pub mod dump;
mod ne_header;

use clap::{App, Arg};

use crate::util::setup_logger;
use crate::multi_error::MultiError;
use crate::app_context::AppContext;



fn main() -> Result<(), MultiError> {

    let app_ctx: AppContext = parse_cmd_line()?;

    setup_logger()?;

    // read the file into memmap

    // parse the file

    // identify the entry point

    // iterate through the assembly

    Ok(())
}

///
/// parse command line arguments
/// 
fn parse_cmd_line() -> Result<AppContext, MultiError> {
    let args = App::new("ne_runner")
        .version("0.1")
        .author("cyrex562")
        .about("")
    .arg(Arg::with_name("input_exe")
        .short("i")
        .long("input_file")
        .help("exe file to load")
        .takes_value(true))
    .get_matches();

    let mut app_ctx: AppContext = Default::default();

    if args.is_present("input_exe") {
        let input_file_str = args.value_of("input_exe").unwrap();
        app_ctx.input_file = input_file_str.to_string();
    }

    Ok(app_ctx)
}