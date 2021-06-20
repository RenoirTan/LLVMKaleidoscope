use crate::{
    error::{Error, ErrorKind, Result},
    token::FileIndex,
};
use std::{
    convert::TryFrom,
    fs::{File, OpenOptions},
    io::{stdin, BufRead, BufReader, Lines, Read, Stdin},
    iter::{Enumerate, Iterator},
    path::Path,
};

/// A file stream which returns a unicode codepoint one at a time.
pub struct FileStream<S: Read> {
    buffer: Enumerate<Lines<BufReader<S>>>,
    line: Vec<char>,
    cursor: usize,
    index: FileIndex,
    error: Option<Error>,
    eof_reached: bool
}

impl<S: Read> FileStream<S> {
    /// Create a new `FileStream` from an iterator over the lines of a buffered
    /// reader.
    pub fn new(buffer: Enumerate<Lines<BufReader<S>>>) -> Self {
        let mut this = FileStream {
            buffer,
            cursor: 0,
            line: Vec::new(),
            index: Default::default(),
            error: None,
            eof_reached: false,
        };
        this.init();
        this
    }

    pub fn eof_reached(&self) -> bool {
        self.eof_reached
    }

    pub fn get_unit(&self) -> Option<char> {
        self.line.get(self.cursor).cloned()
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

    fn init(&mut self) -> &mut Self {
        self.next_line();
        self
    }

    pub fn next_line(&mut self) -> bool {
        if let Some((line_no, line)) = self.buffer.next() {
            self.index = FileIndex::new(Some(line_no), 0);
            match line {
                Ok(l) => {
                    self.line = l.chars().collect();
                    self.cursor = 0;
                    self.eof_reached = false;
                    true
                }
                Err(e) => {
                    self.error = Some(
                        Error::from_err(&e, ErrorKind::FileIOError)
                    );
                    self.eof_reached = true;
                    false
                }
            }
        } else {
            self.line.clear();
            self.eof_reached = true;
            false
        }
    }

    pub fn next_unit(&mut self) -> Option<char> {
        loop {
            if self.eof_reached() {
                break None;
            }
            if self.cursor >= self.line.len() {
                self.next_line();
            } else {
                let unit = self.get_unit();
                self.cursor += 1;
                self.index += 1;
                break unit;
            }
        }
    }
}

impl FileStream<Stdin> {
    pub fn from_stdin() -> Self {
        let stdin = stdin();
        let buffer = BufReader::new(stdin).lines().enumerate();
        let mut this = Self {
            buffer,
            line: Vec::new(),
            cursor: 0,
            index: Default::default(),
            error: None,
            eof_reached: false,
        };
        this.init();
        this
    }
}

impl FileStream<File> {
    /// Create a new `FileStream` from a path.
    pub fn from_path(path: &Path) -> Result<Self> {
        let file = match OpenOptions::new().read(true).open(path) {
            Ok(f) => f,
            Err(e) => return Err(Error::from_err(&e, ErrorKind::FileIOError)),
        };
        let buffer = BufReader::new(file).lines().enumerate();
        let mut this = FileStream {
            buffer,
            cursor: 0,
            line: Vec::new(),
            index: Default::default(),
            error: None,
            eof_reached: false,
        };
        this.init();
        Ok(this)
    }
}

impl TryFrom<&Path> for FileStream<File> {
    type Error = Error;
    fn try_from(path: &Path) -> Result<Self> {
        Self::from_path(path)
    }
}

impl<S: Read> Iterator for FileStream<S> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let unit = self.get_unit();
        self.next_unit();
        unit
    }
}
