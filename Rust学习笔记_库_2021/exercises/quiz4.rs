macro_rules! my_macro {
    ($val:expr) => {
        format!("Hello {}", $val)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_my_macro_world() {
        assert_eq!(my_macro!("world!"), "Hello world!");
    }

    #[test]
    fn test_my_macro_goodbye() {
        assert_eq!(my_macro!("goodbye!"), "Hello goodbye!");
    }
}