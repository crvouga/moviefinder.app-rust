use crate::core::random;

#[derive(Debug, Clone, PartialEq)]
pub enum MediaType {
    Movie,
    Tv,
}

impl MediaType {
    pub fn random() -> MediaType {
        random::choice(vec![MediaType::Movie, MediaType::Tv]).unwrap_or(MediaType::Movie)
    }
}
