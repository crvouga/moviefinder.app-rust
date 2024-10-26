use crate::media::{genre::genre::Genre, tmdb_api};

use super::interface::GenreDb;

pub struct ImplTmdb {
    config: tmdb_api::Config,
}

impl ImplTmdb {
    pub fn new(tmdb_api_read_access_token: String) -> ImplTmdb {
        ImplTmdb {
            config: tmdb_api::Config::new(tmdb_api_read_access_token),
        }
    }
}

impl GenreDb for ImplTmdb {
    fn get_all(&self) -> Result<Vec<Genre>, String> {
        Ok(Vec::new())
    }
}
