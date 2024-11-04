fn _intro() -> &'static str {
    "I'm ready to build a calculator in Rust!"
}

#[cfg(test)]
mod tests {
    use crate::_intro;

    #[test]
    fn test_intro() {
        assert_eq!(_intro(), "I'm ready to build a calculator in Rust!");
    }
}
