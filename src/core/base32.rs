// use alloc::string::String;
// use alloc::vec::Vec;
use core::cmp::min;

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Alphabet {
    Crockford,
    Rfc4648 { padding: bool },
    Rfc4648Lower { padding: bool },
    Rfc4648Hex { padding: bool },
    Rfc4648HexLower { padding: bool },
    Z,
}

const CROCKFORD: &'static [u8] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";
const RFC4648: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
const RFC4648_LOWER: &'static [u8] = b"abcdefghijklmnopqrstuvwxyz234567";
const RFC4648_HEX: &'static [u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUV";
const RFC4648_HEX_LOWER: &'static [u8] = b"0123456789abcdefghijklmnopqrstuv";
const Z: &'static [u8] = b"ybndrfg8ejkmcpqxot1uwisza345h769";

pub fn encode(alphabet: Alphabet, data: &[u8]) -> String {
    let (alphabet, padding) = match alphabet {
        Alphabet::Crockford => (CROCKFORD, false),
        Alphabet::Rfc4648 { padding } => (RFC4648, padding),
        Alphabet::Rfc4648Lower { padding } => (RFC4648_LOWER, padding),
        Alphabet::Rfc4648Hex { padding } => (RFC4648_HEX, padding),
        Alphabet::Rfc4648HexLower { padding } => (RFC4648_HEX_LOWER, padding),
        Alphabet::Z => (Z, false),
    };
    let mut ret = Vec::with_capacity((data.len() + 3) / 4 * 5);

    for chunk in data.chunks(5) {
        let buf = {
            let mut buf = [0u8; 5];
            for (i, &b) in chunk.iter().enumerate() {
                buf[i] = b;
            }
            buf
        };
        ret.push(alphabet[((buf[0] & 0xF8) >> 3) as usize]);
        ret.push(alphabet[(((buf[0] & 0x07) << 2) | ((buf[1] & 0xC0) >> 6)) as usize]);
        ret.push(alphabet[((buf[1] & 0x3E) >> 1) as usize]);
        ret.push(alphabet[(((buf[1] & 0x01) << 4) | ((buf[2] & 0xF0) >> 4)) as usize]);
        ret.push(alphabet[(((buf[2] & 0x0F) << 1) | (buf[3] >> 7)) as usize]);
        ret.push(alphabet[((buf[3] & 0x7C) >> 2) as usize]);
        ret.push(alphabet[(((buf[3] & 0x03) << 3) | ((buf[4] & 0xE0) >> 5)) as usize]);
        ret.push(alphabet[(buf[4] & 0x1F) as usize]);
    }

    if data.len() % 5 != 0 {
        let len = ret.len();
        let num_extra = 8 - (data.len() % 5 * 8 + 4) / 5;
        if padding {
            for i in 1..num_extra + 1 {
                ret[len - i] = b'=';
            }
        } else {
            ret.truncate(len - num_extra);
        }
    }

    String::from_utf8(ret).unwrap()
}

/*
     0,  1,  2,  3,  4,  5,  6,  7,  8,  9,  :,  ;,  <,  =,  >,  ?,  @,  A,  B,  C,
     D,  E,  F,  G,  H,  I,  J,  K,  L,  M,  N,  O,  P,  Q,  R,  S,  T,  U,  V,  W,
     X,  Y,  Z,  [,  \,  ],  ^,  _,  `,  a,  b,  c,  d,  e,  f,  g,  h,  i,  j,  k,
     l,  m,  n,  o,  p,  q,  r,  s,  t,  u,  v,  w,  x,  y,  z,
*/

const CROCKFORD_INV: [i8; 75] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, -1, -1, -1, -1, -1, -1, -1, 10, 11, 12, 13, 14, 15, 16, 17, 1,
    18, 19, 1, 20, 21, 0, 22, 23, 24, 25, 26, -1, 27, 28, 29, 30, 31, -1, -1, -1, -1, -1, -1, 10,
    11, 12, 13, 14, 15, 16, 17, 1, 18, 19, 1, 20, 21, 0, 22, 23, 24, 25, 26, -1, 27, 28, 29, 30,
    31,
];
const RFC4648_INV: [i8; 75] = [
    -1, -1, 26, 27, 28, 29, 30, 31, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8,
    9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1,
];
const RFC4648_INV_PAD: [i8; 75] = [
    -1, -1, 26, 27, 28, 29, 30, 31, -1, -1, -1, -1, -1, 0, -1, -1, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8,
    9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1,
];
const RFC4648_INV_LOWER: [i8; 75] = [
    -1, -1, 26, 27, 28, 29, 30, 31, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    25,
];
const RFC4648_INV_LOWER_PAD: [i8; 75] = [
    -1, -1, 26, 27, 28, 29, 30, 31, -1, -1, -1, -1, -1, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    25,
];
const RFC4648_INV_HEX: [i8; 75] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, -1, -1, -1, -1, -1, -1, -1, 10, 11, 12, 13, 14, 15, 16, 17, 18,
    19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1,
];
const RFC4648_INV_HEX_PAD: [i8; 75] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, -1, -1, -1, 0, -1, -1, -1, 10, 11, 12, 13, 14, 15, 16, 17, 18,
    19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1,
];
const RFC4648_INV_HEX_LOWER: [i8; 75] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 10,
    11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, -1, -1, -1,
    -1,
];
const RFC4648_INV_HEX_LOWER_PAD: [i8; 75] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, -1, -1, -1, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 10,
    11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, -1, -1, -1,
    -1,
];
const Z_INV: [i8; 75] = [
    -1, 18, -1, 25, 26, 27, 30, 29, 7, 31, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, 24, 1, 12, 3, 8, 5, 6, 28, 21, 9, 10, -1, 11, 2, 16, 13, 14, 4, 22, 17, 19, -1, 20, 15, 0,
    23,
];

pub fn decode(alphabet: Alphabet, data: &str) -> Option<Vec<u8>> {
    if !data.is_ascii() {
        return None;
    }
    let data = data.as_bytes();
    let alphabet = match alphabet {
        Alphabet::Crockford => CROCKFORD_INV, // supports both upper and lower case
        Alphabet::Rfc4648 { padding } => {
            if padding {
                RFC4648_INV_PAD
            } else {
                RFC4648_INV
            }
        }
        Alphabet::Rfc4648Lower { padding } => {
            if padding {
                RFC4648_INV_LOWER_PAD
            } else {
                RFC4648_INV_LOWER
            }
        }
        Alphabet::Rfc4648Hex { padding } => {
            if padding {
                RFC4648_INV_HEX_PAD
            } else {
                RFC4648_INV_HEX
            }
        }
        Alphabet::Rfc4648HexLower { padding } => {
            if padding {
                RFC4648_INV_HEX_LOWER_PAD
            } else {
                RFC4648_INV_HEX_LOWER
            }
        }
        Alphabet::Z => Z_INV,
    };
    let mut unpadded_data_length = data.len();
    for i in 1..min(6, data.len()) + 1 {
        if data[data.len() - i] != b'=' {
            break;
        }
        unpadded_data_length -= 1;
    }
    let output_length = unpadded_data_length * 5 / 8;
    let mut ret = Vec::with_capacity((output_length + 4) / 5 * 5);
    for chunk in data.chunks(8) {
        let buf = {
            let mut buf = [0u8; 8];
            for (i, &c) in chunk.iter().enumerate() {
                match alphabet.get(c.wrapping_sub(b'0') as usize) {
                    Some(&-1) | None => return None,
                    Some(&value) => buf[i] = value as u8,
                };
            }
            buf
        };
        ret.push((buf[0] << 3) | (buf[1] >> 2));
        ret.push((buf[1] << 6) | (buf[2] << 1) | (buf[3] >> 4));
        ret.push((buf[3] << 4) | (buf[4] >> 1));
        ret.push((buf[4] << 7) | (buf[5] << 2) | (buf[6] >> 3));
        ret.push((buf[6] << 5) | buf[7]);
    }
    ret.truncate(output_length);
    Some(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let data = "hello world";
        let encoded = encode(Alphabet::Crockford, data.as_bytes());
        let decoded = decode(Alphabet::Crockford, &encoded);
        assert_eq!(decoded, Some(data.as_bytes().to_vec()));
        assert_eq!(
            String::from_utf8(decoded.unwrap()),
            Ok("hello world".to_string())
        );
    }

    #[test]
    fn test_encode_decode_small() {
        let data = "hi";
        let encoded = encode(Alphabet::Crockford, data.as_bytes());
        let decoded = decode(Alphabet::Crockford, &encoded);
        assert_eq!(decoded, Some(data.as_bytes().to_vec()));
    }

    #[test]
    fn test_empty_string() {
        let data = "";
        let encoded = encode(Alphabet::Crockford, data.as_bytes());
        let decoded = decode(Alphabet::Crockford, &encoded);
        assert_eq!(decoded, Some(data.as_bytes().to_vec()));
    }

    #[test]
    fn test_invalid_character() {
        let data = "INVALID==";
        let result = decode(Alphabet::Crockford, data);
        assert_eq!(result, None);
    }
}
