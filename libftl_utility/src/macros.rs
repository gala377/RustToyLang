
#[macro_export]
macro_rules! assert_match {
    ($e:expr, $( $p:pat )+) => {
        assert!(match $e {
            $(
                $p => true,
            ),*
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
    #[should_panic]
    fn panics_when_failure_is_expected() {
        let opt: Option<u8> = Some(0);
        assert_match!(opt, None);
    }
}