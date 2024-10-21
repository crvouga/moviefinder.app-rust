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

    String::from_utf8(encoded).unwrap()
}

pub fn decode(data: &str) -> String {
    let bytes = data.as_bytes();
    let mut decoded = Vec::new();
    let mut buffer = [0u8; 4];

    let decode_table = |b: u8| -> u8 {
        match b {
            b'A'..=b'Z' => b - b'A',
            b'a'..=b'z' => b - b'a' + 26,
            b'0'..=b'9' => b - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            _ => 0,
        }
    };

    for chunk in bytes.chunks(4) {
        for (i, &byte) in chunk.iter().enumerate() {
            buffer[i] = decode_table(byte);
        }

        decoded.push((buffer[0] << 2) | (buffer[1] >> 4));
        if chunk[2] != b'=' {
            decoded.push((buffer[1] << 4) | (buffer[2] >> 2));
        }
        if chunk[3] != b'=' {
            decoded.push((buffer[2] << 6) | buffer[3]);
        }
    }

    String::from_utf8(decoded).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let data = "hello world";
        let encoded = encode(data);
        let decoded = decode(&encoded);
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_encode_decode_padding() {
        let data = "hello";
        let encoded = encode(data);
        let decoded = decode(&encoded);
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_empty_string() {
        let data = "";
        let encoded = encode(data);
        let decoded = decode(&encoded);
        assert_eq!(decoded, data);
    }
}
