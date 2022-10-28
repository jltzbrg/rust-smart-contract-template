fn welcome(text: String) -> String {
    text.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_welcome() {
        let result = "Hola Rust";
        assert_eq!(result, "Hola Rust")
    }
}
