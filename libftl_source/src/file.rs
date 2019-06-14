use std::fs;
use std::io;
use std::io::{
    Read,
};

use crate::Source;

/// TODO - this is just a basic implementation 
/// reading all of the files content to the string.
/// It should be changed as its not really wise
/// to hold a files content in the string
pub struct File {
    _path: String,
    _file: fs::File,
    
    mock: crate::string::String,
}

impl File {

    pub fn new(path: &str) -> io::Result<Self> {
        let p = String::from(path);
        let mut f = fs::File::open(path)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(Self{
            _path: p,
            _file: f,
            mock: crate::string::String::from(&s[..]),
        })
    }

}

impl Source for File {

    type Pointer = crate::string::Pointer;

    fn curr_char(&self) -> Option<char> {
        self.mock.curr_char()
    }

    fn next_char(&mut self) -> Option<char> {
        self.mock.next_char()
    }

    fn curr_ptr(&self) -> Self::Pointer {
        self.mock.curr_ptr()
    }

    fn source_between(&self, begin: &Self::Pointer, end: &Self::Pointer) -> std::string::String {
        self.mock.source_between(begin, end)
    }

}