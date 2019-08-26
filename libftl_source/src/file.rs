//! Holds the implementation of
//! the file source unit and
//! corresponding source pointer.

use std::cell::RefCell;
use std::convert::From;
use std::fs;
use std::io;
use std::io::{Seek, SeekFrom};

use crate::Source;
use ftl_utility::utf8;

pub struct File {
    reader: RefCell<utf8::Reader<fs::File>>,

    current: Option<char>,

    curr_line: usize,
    curr_pos: usize,

    index: u64,
    switch_line: bool,
}

impl File {
    /// Creates new `File` source.
    ///
    /// As opening a file can fail this function
    /// doesn't return `File` and instead
    /// wraps it in the `io::Result` to pass
    /// any error that could happen during opening
    /// a file with
    /// [`std::fs::File::open`](https://doc.rust-lang.org/std/fs/struct.File.html#method.open).
    pub fn new(path: &str) -> io::Result<Self> {
        let f = fs::File::open(path)?;
        Ok(Self::_new(f))
    }

    fn _new(f: fs::File) -> Self {
        let mut s = Self {
            reader: RefCell::new(utf8::Reader::new(f)),
            curr_line: 1,
            curr_pos: 1,
            index: 0,
            switch_line: false,
            current: None,
        };
        s.current = s.next_char();
        s
    }

    fn try_next_char(&mut self) -> Option<char> {
        match self.reader.borrow_mut().read_utf8_char() {
            Ok(ch) => Some(ch),
            Err(err) => {
                if let io::ErrorKind::UnexpectedEof = err.kind() {
                    None
                } else {
                    panic!("Not a valid utf-8 character!");
                }
            }
        }
    }

    fn curr_seek_pos(&self) -> u64 {
        self.reader.borrow_mut().seek(SeekFrom::Current(0)).unwrap()
    }
}

impl From<fs::File> for File {
    fn from(f: fs::File) -> Self {
        Self::_new(f)
    }
}

impl Source for File {
    type Pointer = Pointer;

    fn curr_char(&self) -> Option<char> {
        self.current
    }

    fn next_char(&mut self) -> Option<char> {
        self.current = self.try_next_char();
        if self.current.is_none() {
            return self.current;
        }
        self.index += self.current.unwrap().len_utf8() as u64;
        if self.switch_line {
            self.switch_line = false;
            self.curr_line += 1;
            self.curr_pos = 0;
        }
        if let Some(ch) = self.current {
            if ch == '\n' {
                self.switch_line = true;
            }
            self.curr_pos += 1;
        }
        self.current
    }

    fn curr_ptr(&self) -> Self::Pointer {
        Self::Pointer {
            index: self.index,
            curr_line: self.curr_line,
            curr_pos: self.curr_pos,
        }
    }

    fn source_between(&self, begin: &Self::Pointer, end: &Self::Pointer) -> std::string::String {
        let mut s = String::new();
        let saved_pos = self.curr_seek_pos();
        self.reader
            .borrow_mut()
            .seek(SeekFrom::Start(begin.index))
            .unwrap();
        while self.curr_seek_pos() < end.index {
            s.push(match self.reader.borrow_mut().read_utf8_char() {
                Ok(ch) => ch,
                _ => unreachable!(),
            });
        }
        self.reader
            .borrow_mut()
            .seek(SeekFrom::Start(saved_pos))
            .unwrap();
        s
    }
}

#[derive(Clone)]
pub struct Pointer {
    index: u64,
    curr_line: usize,
    curr_pos: usize,
}

impl crate::Pointer for Pointer {
    fn line(&self) -> usize {
        self.curr_line
    }

    fn position(&self) -> usize {
        self.curr_pos
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::io::{Seek, SeekFrom};

    use tempfile::tempfile;

    use crate::{file::*, tests::*};

    #[test]
    fn source_tests_for_file_source() {
        source_tests(&|raw| {
            let mut temp = tempfile().unwrap();
            write!(&mut temp, "{}", raw).unwrap();
            temp.seek(SeekFrom::Start(0)).unwrap();
            File::from(temp)
        });
    }
}
