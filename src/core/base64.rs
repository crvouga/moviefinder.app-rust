const BASE64_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn encode(data: &str) -> String {
    let bytes = data.as_bytes();
    let mut encoded = Vec::new();
    let mut _padding = 0;

    for chunk in bytes.chunks(3) {
        let mut buffer = [0u8; 3];
        buffer[..chunk.len()].copy_from_slice(chunk);

        encoded.push(BASE64_TABLE[(buffer[0] >> 2) as usize]);
        encoded.push(BASE64_TABLE[((buffer[0] & 0b00000011) << 4 | (buffer[1] >> 4)) as usize]);
        if chunk.len() > 1 {
            encoded.push(BASE64_TABLE[((buffer[1] & 0b00001111) << 2 | (buffer[2] >> 6)) as usize]);
        } else {
            encoded.push(b'=');
            _padding += 1;
        }

        if chunk.len() > 2 {
            encoded.push(BASE64_TABLE[(buffer[2] & 0b00111111) as usize]);
        } else {
            encoded.push(b'=');
            _padding += 1;
        }
    }

    String::from_utf8(encoded).unwrap_or("".to_string())
}
pub fn decode(data: &str) -> Result<String, String> {
    let bytes = data.as_bytes();
    let mut decoded = Vec::new();
    let mut buffer = [0u8; 4];

    let decode_table = |b: u8| -> Option<u8> {
        match b {
            b'A'..=b'Z' => Some(b - b'A'),
            b'a'..=b'z' => Some(b - b'a' + 26),
            b'0'..=b'9' => Some(b - b'0' + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            b'=' => None, // Padding character should not be decoded
            _ => None,    // Any other character is invalid
        }
    };

    for chunk in bytes.chunks(4) {
        if chunk.len() < 4 {
            return Err("Invalid base64 input: chunk is too small".to_string());
        }

        let mut valid_bytes = 0;

        for (i, &byte) in chunk.iter().enumerate() {
            if let Some(decoded_byte) = decode_table(byte) {
                buffer[i] = decoded_byte;
                valid_bytes += 1;
            } else if byte != b'=' {
                return Err(format!("Invalid base64 character: {}", byte));
            }
        }

        // Proceed only with valid non-padding characters
        decoded.push((buffer[0] << 2) | (buffer[1] >> 4));

        if valid_bytes > 2 {
            decoded.push((buffer[1] << 4) | (buffer[2] >> 2));
        }

        if valid_bytes > 3 {
            decoded.push((buffer[2] << 6) | buffer[3]);
        }
    }

    String::from_utf8(decoded).map_err(|e| format!("Invalid UTF-8 sequence: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let data = "hello world";
        let encoded = encode(data);
        let decoded = decode(&encoded);
        assert_eq!(decoded, Ok(data.to_string()));
    }

    #[test]
    fn test_encode_decode_padding() {
        let data = "hello";
        let encoded = encode(data);
        let decoded = decode(&encoded);
        assert_eq!(decoded, Ok(data.to_string()));
    }

    #[test]
    fn test_empty_string() {
        let data = "";
        let encoded = encode(data);
        let decoded = decode(&encoded);
        assert_eq!(decoded, Ok(data.to_string()));
    }
}
