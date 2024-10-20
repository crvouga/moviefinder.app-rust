use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn choice<T>(items: Vec<T>) -> Option<T>
where
    T: Clone,
{
    let mut rng = thread_rng();
    items.as_slice().choose(&mut rng).cloned()
}
