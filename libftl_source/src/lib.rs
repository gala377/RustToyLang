pub mod string;
pub mod file;

/// Represents source containing program source code.
/// After creation the source should point to the 
/// first character in it. 
pub trait Source {

    type Pointer: Pointer;

    /// Returns current character. 
    /// None should be returned if source has ended.
    fn curr_char(&self) -> Option<char>;
    
    /// Shifts the source pointer to the next 
    /// character and returns it.
    /// None should be returned if source has ended.
    fn next_char(&mut self) -> Option<char>;

    /// Returns current source pointer.
    fn curr_ptr(&self) -> Self::Pointer;

    /// Returns source content between two pointers.
    fn source_between(&self, begin: &Self::Pointer, end: &Self::Pointer) -> String;

    /// Returns source content described by span.
    fn source_with_span(&self, span: &Span<Self::Pointer>) -> String {
        self.source_between(&span.beg, &span.end)
    }
}

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
#[derive(Clone, Debug)]
pub struct Span<T: Pointer> {
    /// Beginning of the range.
    pub beg: T,
    /// End of the range.
    pub end: T,
}

pub mod tests {

    use super::*;

    pub fn source_tests<T: Source>(creator: &dyn Fn(&str) -> T) {
        eprintln!("subtest source::tests::string_source_from_empty_str");
        string_source_from_empty_str(creator);
        eprintln!("subtest source::tests::reading_1st_character_from_source");
        reading_1st_character_from_source(creator);
        eprintln!("subtest source::tests::reading_past_empty_source_1");
        reading_past_empty_source_1(creator);
        eprintln!("subtest source::tests::reading_past_empty_source_2");
        reading_past_empty_source_2(creator);
        eprintln!("subtest source::tests::reading_all_characters_from_source");
        reading_all_characters_from_source(creator);
        eprintln!("subtest source::tests::reading_new_line");
        reading_new_line(creator);
        eprintln!("subtest source::tests::reading_past_line");
        reading_past_line(creator);
        eprintln!("subtest source::tests::new_line_at_the_end_of_source");
        new_line_at_the_end_of_source(creator);
        eprintln!("subtest source::tests::reading_white_spaces");
        reading_white_spaces(creator);
        eprintln!("subtest source::tests::reading_multiple_times_past_the_end_of_file");
        reading_multiple_times_past_the_end_of_file(creator);
        eprintln!("subtest source::tests::getting_all_source_with_two_ptr");
        getting_all_source_with_two_ptr(creator);
        eprintln!("subtest source::tests::getting_source_fragment_with_two_ptr");
        getting_source_fragment_with_two_ptr(creator);
    }

    pub fn assert_source<T: Source>(s: &T, ch: Option<char>, line: usize, pos: usize) {
        assert_eq!(s.curr_char(), ch);
        assert_eq!(s.curr_ptr().line(), line);
        assert_eq!(s.curr_ptr().position(), pos);
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
        s.next_char(); s.next_char(); s.next_char();
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
            assert_source(&s, Some(ch), 1, i+1);
            s.next_char();
        }
        s.next_char();
        assert_source(&s, None, 1, raw.len());
    }

    fn reading_new_line<T: Source>(creator: &dyn Fn(&str) -> T) {
        let mut s = creator(r#"a
c"#);
        s.next_char();
        assert_source(&s, Some('\n'), 1, 2);
    }

    fn reading_past_line<T: Source>(creator: &dyn Fn(&str) -> T) {
        let mut s = creator(r#"a
c"#);
        s.next_char(); s.next_char();
        assert_source(&s, Some('c'), 2, 1);
    }

    fn new_line_at_the_end_of_source<T: Source>(creator: &dyn Fn(&str) -> T) {
        let mut s = creator(r#"a
"#);
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
        s.next_char(); s.next_char(); s.next_char();
        assert_source(
            &s, 
            None, 
            raw.lines().count(), 
            raw.lines().last().unwrap().len());
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
        assert_eq!(s.source_between(&beg, &end), raw.chars().skip(10).take(11).collect::<String>());
    }
}