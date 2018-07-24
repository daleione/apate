#[cfg(test)]
mod tests {
    fn to_lowercase<S: Into<String>>(s: S) -> String {
        s.into().to_lowercase()
    }

    #[test]
    fn test_to_lowercase() {
        assert_eq!("lowercase", to_lowercase(String::from("LOWERCASE")));
    }

}
