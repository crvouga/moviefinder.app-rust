use std::{collections::HashMap, hash::Hash};

pub trait VecExt<T> {
    #[allow(dead_code)]
    fn frequencies(self) -> HashMap<T, usize>
    where
        T: Hash + Eq;
}

impl<T> VecExt<T> for Vec<T> {
    fn frequencies(self) -> HashMap<T, usize>
    where
        T: Hash + Eq,
    {
        let mut freq = HashMap::new();
        for item in self {
            *freq.entry(item).or_insert(0) += 1;
        }
        freq
    }
}
