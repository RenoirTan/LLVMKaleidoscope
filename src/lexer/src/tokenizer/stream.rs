use std::{
    convert::TryFrom,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Lines},
    iter::Enumerate,
    path::Path
};
use crate::{
    error::{Result, Error, ErrorKind},
    token::FileIndex
};

/// A file stream which returns a unicode codepoint one at a time.
pub struct Stream {
    buffer: Enumerate<Lines<BufReader<File>>>,
    line: String,
    unit: char,
    index: FileIndex,
    error: Option<Error>
}

impl Stream {
    pub fn new(buffer: Enumerate<Lines<BufReader<File>>>) -> Self {
        let mut this = Self {
            buffer,
            unit: 0 as char,
            line: String::new(),
            index: Default::default(),
            error: None
        };
        this.init();
        this
    }

    pub fn from_path(path: &Path) -> Result<Self> {
        let file = match OpenOptions::new().read(true).open(path) {
            Ok(f) => f,
            Err(e) => return Err(
                Error::from_err(
                    &e,
                    ErrorKind::FileIOError
                )
            )
        };
        let buffer = BufReader::new(file).lines().enumerate();
        let mut this = Self {
            buffer,
            unit: 0 as char,
            line: String::new(),
            index: Default::default(),
            error: None
        };
        this.init();
        Ok(this)
    }

    pub fn get_unit(&self) -> char {
        self.unit
    }

    pub fn get_index(&self) -> FileIndex {
        self.index
    }

    pub fn get_mut_index(&mut self) -> &mut FileIndex {
        &mut self.index
    }

    pub fn get_err(&self) -> Option<Error> {
        self.error.clone()
    }

    pub fn eof_reached(&self) -> bool {
        self.line.is_empty()
    }

    fn init(&mut self) -> &mut Self {
        self.next_line();
        self
    }

    fn next_line(&mut self) -> &mut Self {
        if let Some((line_no, line)) = self.buffer.next() {
            self.index = FileIndex::new(Some(line_no), 0);
            match line {
                Ok(l) => {self.line = l;},
                Err(e) => {
                    self.error = Some(
                        Error::from_err(&e, ErrorKind::FileIOError)
                    );
                }
            }
        } else {
            self.line.clear();
        }
        self
    }
}

impl TryFrom<&Path> for Stream {
    type Error = Error;
    fn try_from(path: &Path) -> Result<Self> {
        Self::from_path(path)
    }
}
