macro_rules! format_regex {
    ($($arg:tt)*) => {{
        std::format!($($arg)*).parse::<regex::Regex>().expect("regex new ok!")
    }}
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_format_regex() {
        assert_eq!(
            format_regex!(r"^{}(.*)", "v").to_string(),
            regex::Regex::new(r"^v(.*)").unwrap().to_string()
        )
    }
}
