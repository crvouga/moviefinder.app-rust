/// Decodes a URL-encoded string by converting `%`-encoded hex values and replacing `+` with spaces.
pub fn decode(encoded: &str) -> String {
    let mut decoded = String::new();
    let mut chars = encoded.chars();

    while let Some(c) = chars.next() {
        if c == '%' {
            // Ensure there are at least two more characters for a valid hex sequence
            let first = chars.next();
            let second = chars.next();
            if let (Some(first), Some(second)) = (first, second) {
                let hex = format!("{}{}", first, second);
                if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                    decoded.push(byte as char);
                }
            }
            // If either `first` or `second` is `None`, skip the invalid `%` sequence
        } else if c == '+' {
            decoded.push(' ');
        } else {
            decoded.push(c);
        }
    }

    decoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_simple() {
        let encoded = "Hello%20World";
        let expected = "Hello World";
        assert_eq!(decode(encoded), expected);
    }

    #[test]
    fn test_decode_with_plus() {
        let encoded = "Hello+World";
        let expected = "Hello World";
        assert_eq!(decode(encoded), expected);
    }

    #[test]
    fn test_decode_with_special_chars() {
        let encoded = "Rust%3A%20safe%2C+fast%2C+and+fun%21";
        let expected = "Rust: safe, fast, and fun!";
        assert_eq!(decode(encoded), expected);
    }

    #[test]
    fn test_decode_empty_string() {
        let encoded = "";
        let expected = "";
        assert_eq!(decode(encoded), expected);
    }

    #[test]
    fn test_decode_partial_escape_sequence() {
        let encoded = "Incomplete%2";
        let expected = "Incomplete"; // `%2` is ignored as it’s incomplete
        assert_eq!(decode(encoded), expected);
    }

    #[test]
    fn test_decode_edge_case_percent_at_end() {
        let encoded = "Edge%";
        let expected = "Edge"; // `%` at the end is ignored
        assert_eq!(decode(encoded), expected);
    }

    #[test]
    fn test_decode_with_equals_sign() {
        let encoded = "key1=value1&key2=value%3Dwith%3Dequals";
        let expected = "key1=value1&key2=value=with=equals";
        assert_eq!(decode(encoded), expected);
    }
}
