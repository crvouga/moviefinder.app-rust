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
