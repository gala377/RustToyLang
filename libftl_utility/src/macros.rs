//! Definitions of helpful macros.


/// Fails if the expression doesn't match 
/// any of the provided patterns. 
/// 
/// Patterns are checked in the same order as 
/// they were passed to the macro.
/// 
/// # Arguments:
/// 
/// * `e` - Expression to be matched.
/// * `p` - One or more patterns to match the expression against.
/// 
/// # Examples
/// Matching on the `Option`: 
/// 
/// ```rust,should_panic
/// # #[macro_use] extern crate ftl_utility;
/// # use ftl_utility::assert_match;
/// # fn main() {
/// let val = Some(5);
/// 
/// assert_match!(val, None, Some(5)); // matches Some(5) or None
/// assert_match!(val, None); // this fails
/// assert_match!(val, Some(_)); // this passes
/// # }
/// ```
#[macro_export]
macro_rules! assert_match {
    ($e:expr $(, $p:pat)+) => {
        assert!(match $e {
            $(
                $p => true,
            )*
            _ => false,
        });
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn simple_match_passes() {
        assert_match!(1, 1);       
    }

    #[test]
    fn match_on_enums() {
        let opt: Option<u8> = None;
        assert_match!(opt, None);
    }

    #[test]
    fn match_multiple_patterns() {
        let opt: Option<u8> = None;
        assert_match!(opt, Some(1), None);
    }

    #[test]
    #[should_panic]
    fn panics_when_failure_is_expected_on_single_pattern() {
        let opt: Option<u8> = Some(0);
        assert_match!(opt, None);
    }

    #[test]
    #[should_panic]
    fn panics_when_failure_is_expected_on_multiple_patterns() {
        let opt: Option<u8> = Some(0);
        assert_match!(opt, None, Some(1), Some(3...5));
    }
}