mod test_error {
    use compiler::util::RaptorexError;
    #[macro_use]
    use compiler::util::throw_error;

    #[test]
    fn test_error() {
        let error = throw_error!("This", "is", "my", "error");
        assert_eq!(error.message, String::from("This is my error"));
    }
}
