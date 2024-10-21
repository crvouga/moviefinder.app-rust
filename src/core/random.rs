use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

pub fn choice<T>(items: Vec<T>) -> Option<T>
where
    T: Clone,
{
    let mut rng = thread_rng();
    items.as_slice().choose(&mut rng).cloned()
}

pub fn bool() -> bool {
    let mut rng = thread_rng();
    rng.gen()
}
