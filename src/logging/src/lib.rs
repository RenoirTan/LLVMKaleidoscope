//! A sub-crate for handling the logging needs of this crate.


use std::{convert::AsRef, fs::OpenOptions, path::Path};

use env_logger::Target;
// use log::LevelFilter;

use crate::{
    error::{Error, ErrorKind, Result},
    helpers::format_record
};

pub mod error;
pub mod helpers;

/// Initialise the logger used by LLVMKaleidoscope. This should only be called
/// once and called at the beginning of the "main" function after parsing
/// command line arguments.
pub fn init(output_filepath: Option<&dyn AsRef<Path>>) -> Result<()> {
    let target = if let Some(path) = output_filepath {
        let path = path.as_ref();
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .map_err(Error::factory(ErrorKind::Other))?;
        Target::Pipe(Box::new(file))
    } else {
        Target::Stderr
    };

    env_logger::builder()
        .is_test(true)
        .format(format_record)
        // .filter_level(LevelFilter::Warn)
        .target(target)
        .try_init()
        .map_err(Error::factory(ErrorKind::LoggerError))
}
