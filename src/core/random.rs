use std::time::{SystemTime, UNIX_EPOCH};

pub fn choice<T>(items: Vec<T>) -> Option<T>
where
    T: Clone,
{
    if items.is_empty() {
        return None;
    }

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let nanos = now.as_nanos();
    let index = (nanos % items.len() as u128) as usize;

    Some(items[index].clone())
}

pub fn bool() -> bool {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    now.as_nanos() % 2 == 0
}

pub fn unit() -> f32 {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let nanos = now.as_nanos();
    let max_nanos = 1_000_000_000;
    let unit = (nanos % max_nanos) as f32;

    unit / max_nanos as f32
}

pub fn string(length: usize) -> String {
    let mut result = String::new();

    let mut num = (unit() * f32::powi(length as f32, 10)) as u64;

    while num > 0 {
        let digit = (num % 36) as u8;
        let char = if digit < 10 {
            (b'0' + digit) as char
        } else {
            (b'a' + (digit - 10)) as char
        };
        result.push(char);
        num /= 36;
    }

    result.chars().rev().collect::<String>()
}
