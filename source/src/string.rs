use std::{
    convert::From,
};
use crate::Source;

/// Program source given as the 
/// hardcoded string or just std::string::String in general.
pub struct String {    
    /// Raw source as String.
    raw: std::string::String,
    
    curr_line: usize,
    curr_pos: usize,

    /// Current pointer position. 
    /// Corresponds to the current character starting at 0.
    index: usize,

    /// Should the curr_line be incremented in the next 
    /// pointer shift. 
    switch_line: bool,
}


impl From<&'_ str> for String {
    fn from(raw:  &str) -> Self {
        let mut s = Self {
            raw: std::string::String::from(raw),
            curr_line: 1,
            curr_pos: 1,
            index: 0, 
            switch_line: false,
        };
        if let Some('\n') = s.curr_char() {
            s.switch_line = true;
        }
        s
    }
}

impl From<std::string::String> for String {
    fn from(raw: std::string::String) -> Self {
        let mut s = Self {
            raw,
            curr_line: 1,
            curr_pos: 1,
            index: 0,
            switch_line: false,
        };
        if let Some('\n') = s.curr_char() {
            s.switch_line = true;
        }
        s
    }
}

impl Source for String {

    type Pointer = Pointer;

    fn curr_char(&self) -> Option<char> {
        self.raw.chars()
                .nth(self.index)
    }

    fn next_char(&mut self) -> Option<char> {
        if self.index == self.raw.len() {
            return None;
        }
        self.index += 1;
        let opt = self.curr_char();
        if self.switch_line {
            self.switch_line = false;
            self.curr_line += 1;
            self.curr_pos = 0;
        }
        if let Some(ch) = opt {
            if ch == '\n' {
                self.switch_line = true;
            }                
            self.curr_pos += 1;
        }
        opt
    }

    fn curr_ptr(&self) -> Self::Pointer {
        Self::Pointer {
            index: self.index, 
            curr_line: self.curr_line,
            curr_pos: self.curr_pos,
        }
    }

    fn source_between(&self, begin: &Self::Pointer, end: &Self::Pointer) -> std::string::String {
        self.raw.chars()
                .skip(begin.index)
                .take(end.index - begin.index + 1)
                .collect()
    }   
}   

#[derive(Clone, Debug)]
pub struct Pointer {
    index: usize,

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
    use crate::{
            string::*,
            tests::*,
    };
    
    #[test] 
    fn string_source_from_empty_str() {
        let s = String::from("");
        assert_source(&s, None, 1, 1);
    }

    #[test]
    fn string_source_from_empty_string() {
        let string = std::string::String::new();
        let s = String::from(string);
        assert_source(&s, None, 1, 1);
    }

    #[test]
    fn source_tests_for_string_source() {
        source_tests(&|raw| String::from(raw));
    }
}