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

/// Encodes a string into a URL-encoded format by replacing special characters with `%`-encoded hex values
/// and replacing spaces with `+`.
pub fn encode(input: &str) -> String {
    let mut encoded = String::new();

    for c in input.chars() {
        match c {
            ' ' => encoded.push('+'),
            // Encode characters that are not alphanumeric or safe symbols (-, _, ., ~)
            c if c.is_ascii_alphanumeric() || "-_.~".contains(c) => encoded.push(c),
            c => {
                // Encode other characters as `%XX` per UTF-8 bytes
                for byte in c.to_string().as_bytes() {
                    encoded.push_str(&format!("%{:02X}", byte));
                }
            }
        }
    }

    encoded
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

    #[test]
    fn test_encode_simple() {
        let input = "Hello World";
        let expected = "Hello+World";
        assert_eq!(encode(input), expected);
    }

    #[test]
    fn test_encode_with_special_chars() {
        let input = "Rust: safe, fast, and fun!";
        let expected = "Rust%3A+safe%2C+fast%2C+and+fun%21";
        assert_eq!(encode(input), expected);
    }

    #[test]
    fn test_encode_empty_string() {
        let input = "";
        let expected = "";
        assert_eq!(encode(input), expected);
    }

    #[test]
    fn test_encode_with_equals_sign() {
        let input = "key1=value1&key2=value=with=equals";
        let expected = "key1%3Dvalue1%26key2%3Dvalue%3Dwith%3Dequals";
        assert_eq!(encode(input), expected);
    }

    #[test]
    fn test_encode_with_unicode() {
        let input = "Rust 💖 safe";
        let expected = "Rust+%F0%9F%92%96+safe"; // Unicode characters are properly encoded
        assert_eq!(encode(input), expected);
    }

    #[test]
    fn test_encode_with_alphanumeric() {
        let input = "abc123";
        let expected = "abc123"; // Alphanumeric characters are not encoded
        assert_eq!(encode(input), expected);
    }

    #[test]
    fn test_encode_with_safe_symbols() {
        let input = "-_.~";
        let expected = "-_.~"; // Safe symbols are not encoded
        assert_eq!(encode(input), expected);
    }
}
