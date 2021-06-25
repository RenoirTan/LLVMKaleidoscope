use crate::{
    error::{Error, ErrorKind, Result},
    token::FileIndex,
};
use std::{
    convert::TryFrom,
    fs::OpenOptions,
    io::{stdin, BufRead, BufReader, Lines, Read},
    iter::{Enumerate, Iterator},
    path::Path,
};

type BufferIterator = Enumerate<Lines<BufReader<Box<dyn Read>>>>;

/// A file stream which returns a unicode codepoint one at a time.
pub struct FileStream {
    buffer: BufferIterator,
    line: Vec<char>,
    cursor: usize,
    index: FileIndex,
    error: Option<Error>,
    eof_reached: bool
}

impl FileStream {
    /// Create a new `FileStream` from an iterator over the lines of a buffered
    /// reader.
    pub fn new(buffer: BufferIterator) -> Self {
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

    /// Check if the file/stream has ended.
    pub fn eof_reached(&self) -> bool {
        self.eof_reached
    }

    /// Get the character currently being read.
    pub fn get_unit(&self) -> Option<char> {
        self.line.get(self.cursor).cloned()
    }

    /// Get the index of the current character.
    pub fn get_index(&self) -> FileIndex {
        self.index
    }

    /// Get a mutable reference to the index of the current character.
    #[warn(unsafe_code)]
    pub fn get_mut_index(&mut self) -> &mut FileIndex {
        &mut self.index
    }

    /// Get the error currently being stored.
    pub fn get_err(&self) -> Option<Error> {
        self.error.clone()
    }

    /// Set the current error to [`None`] and return the old error.
    pub fn silence_err(&mut self) -> Option<Error> {
        self.error.take()
    }

    fn init(&mut self) -> &mut Self {
        self.next_line();
        self
    }

    /// Push the next line into the buffer.
    /// If successful, [`true`] is returned.
    /// Otherwise, if the file/stream has ended or an error has happened,
    /// [`false`] is returned.
    pub fn next_line(&mut self) -> bool {
        if let Some((line_no, line)) = self.buffer.next() {
            match line {
                Ok(l) => {
                    self.line = l.chars().collect();
                    // Mandatory extra new line character so that
                    // the tokeniser knows that the end of the line has
                    // been reached.
                    // Useful for getting to the end of a comment or statement
                    self.line.push('\n');
                    self.cursor = 0;
                    self.eof_reached = false;
                    self.index = FileIndex::new(Some(line_no), 0);
                    true
                }
                Err(e) => {
                    self.error = Some(
                        Error::from_err(Box::new(e), ErrorKind::FileIOError)
                    );
                    self.eof_reached = false;
                    false
                }
            }
        } else {
            self.line.clear();
            self.eof_reached = true;
            false
        }
    }

    /// Read the next character in the stream.
    /// If there are no more characters or an error has occurred,
    /// [`None`] is returned.
    pub fn next_unit(&mut self) -> Option<char> {
        loop {
            if self.eof_reached() || self.error.is_some() {
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

    pub fn from_stdin() -> Self {
        let stdin: Box<dyn Read> = Box::new(stdin());
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

    /// Create a new `FileStream` from a path.
    pub fn from_path(path: &Path) -> Result<Self> {
        let file: Box<dyn Read> = match OpenOptions::new()
            .read(true)
            .open(path)
        {
            Ok(f) => Box::new(f),
            Err(e) => return Err(
                Error::from_err(Box::new(e), ErrorKind::FileIOError)
            ),
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

impl Default for FileStream {
    fn default() -> Self {
        Self::from_stdin()
    }
}

impl TryFrom<&Path> for FileStream {
    type Error = Error;
    fn try_from(path: &Path) -> Result<Self> {
        Self::from_path(path)
    }
}

impl Iterator for FileStream {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let unit = self.get_unit();
        self.next_unit();
        unit
    }
}
