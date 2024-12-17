use std::time::{SystemTime, UNIX_EPOCH};

pub fn v4() -> String {
    let mut bytes = [0u8; 16];
    fill_bytes_with_time(&mut bytes);

    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;

    format!(
        "{:08x}-{:04x}-{:04x}-{:04x}-{:04x}{:08x}",
        u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
        u16::from_be_bytes([bytes[4], bytes[5]]),
        u16::from_be_bytes([bytes[6], bytes[7]]),
        u16::from_be_bytes([bytes[8], bytes[9]]),
        u16::from_be_bytes([bytes[10], bytes[11]]),
        u32::from_be_bytes([bytes[12], bytes[13], bytes[14], bytes[15]])
    )
}

fn fill_bytes_with_time(bytes: &mut [u8]) {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let nanos = now.as_nanos().to_le_bytes();

    for (i, byte) in bytes.iter_mut().enumerate() {
        *byte = nanos[i % nanos.len()];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v4_format() {
        let uuid = v4();

        assert_eq!(uuid.len(), 36);

        assert_eq!(uuid.chars().nth(8), Some('-'));
        assert_eq!(uuid.chars().nth(13), Some('-'));
        assert_eq!(uuid.chars().nth(18), Some('-'));
        assert_eq!(uuid.chars().nth(23), Some('-'));

        assert_eq!(uuid.chars().nth(14), Some('4'));

        let variant_digit = uuid.chars().nth(19).unwrap().to_digit(16).unwrap();
        assert_eq!(variant_digit & 0b1100, 0b1000);
    }
}
