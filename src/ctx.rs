use crate::media::media_db;

pub struct Ctx {
    pub media_db: Box<dyn media_db::MediaDb>,
}

impl Ctx {
    pub fn new(tmdb_api_read_access_token: String) -> Ctx {
        let media_db = Box::new(media_db::tmdb_movie::TmdbMovie::new(
            tmdb_api_read_access_token,
        ));
        let ctx = Ctx { media_db };
        ctx
    }
}
