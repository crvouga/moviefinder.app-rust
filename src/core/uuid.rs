use rand::Rng;

pub fn v4() -> String {
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 16];
    rng.fill(&mut bytes);

    bytes[6] = (bytes[6] & 0x0f) | 0x40; // Version 4
    bytes[8] = (bytes[8] & 0x3f) | 0x80; // Variant (RFC 4122)

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v4_format() {
        let uuid = v4();
        // Check length of the generated UUID (should be 36 with hyphens)
        assert_eq!(uuid.len(), 36);

        // Check if the hyphens are in the correct positions
        assert_eq!(uuid.chars().nth(8), Some('-'));
        assert_eq!(uuid.chars().nth(13), Some('-'));
        assert_eq!(uuid.chars().nth(18), Some('-'));
        assert_eq!(uuid.chars().nth(23), Some('-'));

        // Check if the version is '4'
        assert_eq!(uuid.chars().nth(14), Some('4'));

        // Check if the variant is correct (most significant bits of byte 8 should be '10')
        let variant_digit = uuid.chars().nth(19).unwrap().to_digit(16).unwrap();
        assert_eq!(variant_digit & 0b1100, 0b1000);
    }
}
