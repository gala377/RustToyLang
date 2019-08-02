//! Holds implementation of the 
//! [`String`](struct.String.html)
//!  source and its corresponding pointer.

use std::{
    convert::From,
};
use crate::Source;

/// Program source given as the 
/// hardcoded string or just
/// [`std::string::String`](https://doc.rust-lang.org/std/string/struct.String.html)
/// in general.
/// 
/// [`From`](https://doc.rust-lang.org/std/convert/trait.From.html)
/// trait is implemented for the 
/// `&str` and [`std::string::String`](https://doc.rust-lang.org/std/string/struct.String.html)
/// types for convenient use.
/// 
/// There is nothing noteworthy about this implementation.
/// Source keeps raw program source as the 
/// [`std::string::String`](https://doc.rust-lang.org/std/string/struct.String.html).
/// 
/// Current position in based on the index, which actually 
/// doesn't index raw string directly (because of the possible 
/// multibyte unicode characters). Instead it represents 
/// how many characters should be skipped, as the characters 
/// are retrieved through 
/// [`chars`](https://doc.rust-lang.org/std/string/struct.String.html#method.chars)
///  method and iterators 
/// [`skip`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.skip)
/// method.
///
/// Some gymnastics need to be done when dealing with the new lines
/// so the source tracks them correctly but thats about it.
///  
/// # Pointer 
///
/// As stated before pointer implementation for this struct 
/// hold hom many characters should be skipped using 
/// [`skip`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.skip)
/// method od characters iterator. 
/// 
/// Additionaly informations about line number and 
/// number of the character in line pointer points 
/// to are also being kept as fields of the pointer 
/// struct as its not possible to know them based 
/// only on the character index.   
/// 
/// # Examples
/// 
/// Creating and using source:
/// 
/// ```
/// use ftl_source::string::String;
/// use ftl_source::Source;
/// 
/// let mut source = String::from("def add a b: a + b");
/// match source.curr_char() {
///     Some('d') => (),
///     _ => panic!("Wrong character!"),  
/// };
/// match source.next_char() {
///     Some('e') => (),
///     _ => panic!("Wrong character!"),  
/// };
/// ```
/// 
/// Using pointers to retrieve source fragment:
/// 
/// ```
/// # use ftl_source::{Source, Pointer, string::String};
/// # let mut source = String::from("
/// #    def add a b: a + b
/// # ");
/// let beg = source.curr_ptr();
/// for _ in 0..3 {
///     source.next_char();
/// }
/// let end = source.curr_ptr();
/// println!("{}", source.source_between(&beg, &end));
/// ```
pub struct String {    
    /// Raw source as String.
    raw: std::string::String,
    
    curr_line: usize,
    curr_pos: usize,

    /// Current pointer position. 
    /// Corresponds to the current character, starting at 0.
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

/// Source pointer for the [`String`](struct.String.html)
/// source.
/// 
/// See [`String`](struct.String.html) source for more 
/// information about implementation.
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