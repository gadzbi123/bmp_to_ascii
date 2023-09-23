#[cfg(test)]
mod tests {
    use crate::validate_size;

    #[test]
    fn no_x() {
        assert!(validate_size("123s123").is_err())
    }
    #[test]
    fn invalid_number() {
        assert!(validate_size("12ax123").is_err());
        assert!(validate_size("12x12b").is_err());
        assert!(validate_size("123x-123").is_err());
        assert!(validate_size("-123x123").is_err());
        assert!(validate_size("123x123x5").is_err());
    }
    #[test]
    fn valid_size() {
        assert_eq!(validate_size("123x123").unwrap(), (123, 123));
        assert_eq!(validate_size("123x123").unwrap(), (123, 123))
    }
}
