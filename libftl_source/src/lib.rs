//! This module defines structs and traits that
//! deal with the source for the rest of the
//! compilation process.
//!
//! Source is any unit that could hold a source code of
//! the program written in the FTL language.
//!
//! Module comes with two out of the box implementations
//! of the source units. [`File`](file/struct.File.html)
//! being file in the filesystem,
//! and [`String`](string/struct.String.html)
//! being incode, hardcoded string, which is
//! nice for testing.
//!
//! Rest of the FTL libraries use
//! [`Source`](trait.Source.html) trait
//! (and related to it [`Pointer`](trait.Pointer.html) trait) as an
//! abstraction over source.
//! No assumptions are made over the way
//! the object should fetch characters so there
//! can be implementations that fetch them from
//! web or wait for user input.

use log::info;

pub mod file;
pub mod string;

/// Represents source containing program source code.
///
/// After creation the source should point to the
/// first character in it.
///
/// # Examples
///
/// Example implementations can be found
/// under [`String`](string/struct.String.html) and
/// [`File`](file/struct.File.html) sources.
pub trait Source {
    /// See [`Pointer`](trait.Pointer.html) trait.
    type Pointer: Pointer;

    /// Returns current character.
    /// None should be returned if source has ended.
    fn curr_char(&self) -> Option<char>;

    /// Shifts the source pointer to the next
    /// character and returns it.
    /// None should be returned if source has ended.
    fn next_char(&mut self) -> Option<char>;

    /// Returns pointer to the current place in the source.
    fn curr_ptr(&self) -> Self::Pointer;

    /// Returns copy of the source content between two pointers.
    fn source_between(&self, begin: &Self::Pointer, end: &Self::Pointer) -> String;

    /// Returns copy of the sources content described by span.
    fn source_with_span(&self, span: &Span<Self::Pointer>) -> String {
        self.source_between(&span.beg, &span.end)
    }
}

/// Pointer represent place in corresponding source.
///
/// Each source should have an implementation of
/// the pointer to it. Pointers are used to retrieve
/// copies of the parts of the source for error
/// and compile time messages.
///
/// # Examples
///
/// Examples can be found in [`String`](string/struct.String.html)
/// source implementation of its corresponding
/// pointer struct:
/// [`Pointer`](string/struct.Pointer.html).
///
/// Requires [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html)
/// implementation as its often needed do clone span of some syntax
/// structure when modyfing it.
pub trait Pointer: Clone {
    /// Returns line number, starting from one, from the
    /// place in the source the pointer is pointing to.
    fn line(&self) -> usize;

    /// Returns number of the character in line,
    /// starting from one, from the place in the source
    /// the pointer is pointing to.
    fn position(&self) -> usize;
}

/// Represents range in source.
///
/// Mostly used to represent range in code in which
/// concrete symbol or some syntax unix is defined.
///
/// Its important to properly handle span when
/// modyfing syntax tree in mutable passes because
/// resulting error messeges could be unhelpful
/// or even straight down confusing.
///
/// # Examples
///
/// Cloning span information to retain insource range
/// after modyfing ast node value:
///
/// ```ignore
/// fn reverse_identifier<P: Pointer>(ident: &Ident<P>) -> Ident<P> {
///     Ident {
///         id: ident.id,
///         span: ident.span.clone(),
///         symbol: ident.symbol.chars().rev().collect(),
///     }
/// }
/// ```
#[derive(Clone, Debug)]
pub struct Span<T: Pointer> {
    /// Beginning of the range.
    pub beg: T,
    /// End of the range.
    pub end: T,
}

/// Generic test functions to test the
/// [`Source`](struct.Source.html)
/// implementations with.
///
/// Implementors should make one test which
/// runs [`source_tests`](fn.source_tests.html) function.
///
/// Additionaly there is one helper function
/// [`assert_source`](fn.assert_source.html) which
/// performs several checks on source state at the same time
/// so only one test is needed.
pub mod tests {

    use super::*;

    /// Perform sequence of generic unit tests for the source.
    ///
    /// # Arguments
    /// * creator - factory function creating instance of the
    ///     tested source from the passed `&str` content.
    pub fn source_tests<T: Source>(creator: &dyn Fn(&str) -> T) {
        info!("subtest source::tests::string_source_from_empty_str");
        string_source_from_empty_str(creator);
        info!("subtest source::tests::reading_1st_character_from_source");
        reading_1st_character_from_source(creator);
        info!("subtest source::tests::reading_past_empty_source_1");
        reading_past_empty_source_1(creator);
        info!("subtest source::tests::reading_past_empty_source_2");
        reading_past_empty_source_2(creator);
        info!("subtest source::tests::reading_all_characters_from_source");
        reading_all_characters_from_source(creator);
        info!("subtest source::tests::reading_new_line");
        reading_new_line(creator);
        info!("subtest source::tests::reading_past_line");
        reading_past_line(creator);
        info!("subtest source::tests::new_line_at_the_end_of_source");
        new_line_at_the_end_of_source(creator);
        info!("subtest source::tests::reading_white_spaces");
        reading_white_spaces(creator);
        info!("subtest source::tests::reading_multiple_times_past_the_end_of_file");
        reading_multiple_times_past_the_end_of_file(creator);
        info!("subtest source::tests::getting_all_source_with_two_ptr");
        getting_all_source_with_two_ptr(creator);
        info!("subtest source::tests::getting_source_fragment_with_two_ptr");
        getting_source_fragment_with_two_ptr(creator);
    }

    /// Asserts source state equals the one passed in the arguments.
    ///
    /// There are three asserts being made:
    /// * result of the [`curr_char`](../trait.Source#method.curr_char) method;
    /// * line of the currently pointed character;
    /// * in line position of the currently pointed character;
    pub fn assert_source<T: Source>(s: &T, ch: Option<char>, line: usize, pos: usize) {
        assert_eq!(
            s.curr_char(),
            ch,
            "Expected different character (right is expected one)"
        );
        assert_eq!(
            s.curr_ptr().line(),
            line,
            "Wrong line number (right is expected one)"
        );
        assert_eq!(
            s.curr_ptr().position(),
            pos,
            "Wrong in line position (right is expected one)"
        );
    }

    fn string_source_from_empty_str<T: Source>(creator: &dyn Fn(&str) -> T) {
        let s = creator("");
        assert_source(&s, None, 1, 1);
    }

    fn reading_past_empty_source_1<T: Source>(creator: &dyn Fn(&str) -> T) {
        let mut s = creator("");
        s.next_char();
        assert_source(&s, None, 1, 1);
    }

    fn reading_past_empty_source_2<T: Source>(creator: &dyn Fn(&str) -> T) {
        let mut s = creator("");
        s.next_char();
        s.next_char();
        s.next_char();
        assert_source(&s, None, 1, 1);
    }

    fn reading_1st_character_from_source<T: Source>(creator: &dyn Fn(&str) -> T) {
        let s = creator("abcde");
        assert_source(&s, Some('a'), 1, 1);
    }

    fn reading_all_characters_from_source<T: Source>(creator: &dyn Fn(&str) -> T) {
        let raw = "abcde";
        let mut s = creator(&raw);
        for (i, ch) in raw.chars().enumerate() {
            assert_source(&s, Some(ch), 1, i + 1);
            s.next_char();
        }
        s.next_char();
        assert_source(&s, None, 1, raw.len());
    }

    fn reading_new_line<T: Source>(creator: &dyn Fn(&str) -> T) {
        let mut s = creator(
            r#"a
c"#,
        );
        s.next_char();
        assert_source(&s, Some('\n'), 1, 2);
    }

    fn reading_past_line<T: Source>(creator: &dyn Fn(&str) -> T) {
        let mut s = creator(
            r#"a
c"#,
        );
        s.next_char();
        s.next_char();
        assert_source(&s, Some('c'), 2, 1);
    }

    fn new_line_at_the_end_of_source<T: Source>(creator: &dyn Fn(&str) -> T) {
        let mut s = creator(
            r#"a
"#,
        );
        s.next_char();
        assert_source(&s, Some('\n'), 1, 2);
        s.next_char();
        assert_source(&s, None, 2, 0);
    }

    fn reading_white_spaces<T: Source>(creator: &dyn Fn(&str) -> T) {
        let raw = "a b";
        let mut s = creator(&raw);
        s.next_char();
        assert_source(&s, Some(' '), 1, 2);
    }

    fn reading_multiple_times_past_the_end_of_file<T: Source>(creator: &dyn Fn(&str) -> T) {
        let raw = r#"
        asdas
        asdas
        fdsfsdfds
        "#;
        let mut s = creator(&raw);
        for _ in raw.chars() {
            s.next_char();
        }
        s.next_char();
        s.next_char();
        s.next_char();
        assert_source(
            &s,
            None,
            raw.lines().count(),
            raw.lines().last().unwrap().len(),
        );
    }

    fn getting_all_source_with_two_ptr<T: Source>(creator: &dyn Fn(&str) -> T) {
        let raw = r#"
        asdas
        gtrhdrthytj ydtj ukf 
        ey ste e 
        a"#;
        let mut s = creator(&raw);
        let beg = s.curr_ptr();
        for _ in raw.chars() {
            s.next_char();
        }
        let end = s.curr_ptr();
        assert_eq!(s.source_between(&beg, &end), raw);
    }
    fn getting_source_fragment_with_two_ptr<T: Source>(creator: &dyn Fn(&str) -> T) {
        let raw = r#"
        asdas
        gtrhdrthytj ydtj ukf 
        ey ste e 
        a"#;
        let mut s = creator(&raw);
        for _ in raw.chars().take(10) {
            s.next_char();
        }
        let beg = s.curr_ptr();
        for _ in raw.chars().skip(10).take(10) {
            s.next_char();
        }
        let end = s.curr_ptr();
        assert_eq!(
            s.source_between(&beg, &end),
            raw.chars().skip(10).take(10).collect::<String>()
        );
    }
}
