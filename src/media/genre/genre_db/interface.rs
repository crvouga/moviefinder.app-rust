use crate::media::genre::genre::Genre;

pub trait GenreDb {
    fn get_all(&self) -> Result<Vec<Genre>, String>;
}
