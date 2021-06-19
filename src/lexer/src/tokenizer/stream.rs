use std::{fs::File, io::{/* BufRead,  */BufReader}, path::Path};
use super::{Result, Error, ErrorKind};

pub struct Stream {
    pub file: BufReader<File>,
    pub unit: char
}

impl Stream {
    pub fn new(path: &Path) -> Result<Self> {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) => return Err(
                Error::new(
                    format!("{}", e),
                    ErrorKind::FileIOError
                )
            )
        };
        let file = BufReader::new(file);
        Ok(Self {
            file,
            unit: 0 as char
        })
    }
}
