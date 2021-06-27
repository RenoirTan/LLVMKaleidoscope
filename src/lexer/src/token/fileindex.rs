//! A representation of the location of a character in a file.
//! 
//! See [`FileIndex`] for more information.

use std::ops::{Add, AddAssign, Sub, SubAssign};
use serde::{Serialize, Deserialize};

/// Represents the location of a character
/// (i.e. displayed glyphs, diacritics are counted as separate symbols)
/// in a file or stream.
///
/// This index allows you to store the line on which the character sits
/// (i.e. how many newlines have passed) if you have that data and the
/// column of the characters (i.e. how many characters there are before it).
/// If the character is the first of many in a line, then its column will be 0.
///
/// If you don't know how many lines have passed, or if that data is
/// irrelevant, you can set [`FileIndex::line`] to `None` in
/// [`FileIndex::new`].
///
/// You can also change the column of a `FileIndex` object using the
/// add (`+`), add_assign (`+=`), sub (`-`) and sub_assign (`-=`) operators
/// since I've so handily implemented the traits that code for these
/// operators.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct FileIndex {
    line: Option<usize>,
    column: usize,
}

impl FileIndex {
    /// Create a new [`FileIndex`] object.
    pub fn new(line: Option<usize>, column: usize) -> Self {
        Self { line, column }
    }

    /// Get the line of the byte.
    pub fn get_line(&self) -> Option<usize> {
        self.line
    }

    /// Get the line of the byte but treat an unknown line number as 0.
    pub fn get_line_number(&self) -> usize {
        self.line.unwrap_or(0)
    }

    /// Get the column of the byte.
    pub fn get_column(&self) -> usize {
        self.column
    }

    /// Get the position of a character after a newline.
    ///
    /// If the number of lines passed has been specified, then the line
    /// of the new `FileIndex` object will be the number of lines that have
    /// already been passed plus one, and the column will be 0.
    ///
    /// If the number of lines is unknown, the value of the new column
    /// will be the value of the original column plus the length of the
    /// newline character or sequence in bytes.
    /// The length of this newline sequence must be supplied by you and varies
    /// by OS.
    /// If you are on Linux, the newline sequence is '\n', so `newline_length`
    /// should be 1.
    /// However, if you are on Windows, the newline sequence is '\r\n', so
    /// `newline_length` should be 2.
    ///
    /// # Example
    ///
    /// ```
    /// use kaleidoscope_lexer::token::FileIndex;
    /// 
    /// let fi_1 = FileIndex::new(None, 5); // 5 characters have passed
    /// // OwO what's this? A new line?
    /// // This newline sequence is 2 characters long.
    /// // You can call str.len() safely here because '\r\n' are both
    /// // 1-byte ASCII characters.
    /// let fi_2 = fi_1.newline("\r\n".len());
    /// assert!(fi_2.get_column() == 7);
    /// ```
    pub fn newline(&self, newline_length: usize) -> Self {
        let (line, column) = match self.line {
            Some(l) => (Some(l + 1), 0),
            None => (None, self.column + newline_length),
        };
        Self { line, column }
    }
}

impl std::fmt::Display for FileIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<line: {}, col: {}>",
            self.get_line_number(),
            self.get_column()
        )
    }
}

impl Default for FileIndex {
    fn default() -> Self {
        Self {
            line: None,
            column: 0,
        }
    }
}

impl Add<usize> for FileIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self {
            line: self.get_line(),
            column: self.get_column() + rhs,
        }
    }
}

impl AddAssign<usize> for FileIndex {
    fn add_assign(&mut self, rhs: usize) {
        self.column += rhs;
    }
}

impl Sub<usize> for FileIndex {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self::Output {
        Self {
            line: self.get_line(),
            column: self.get_column() - rhs,
        }
    }
}

impl SubAssign<usize> for FileIndex {
    fn sub_assign(&mut self, rhs: usize) {
        self.column -= rhs;
    }
}
