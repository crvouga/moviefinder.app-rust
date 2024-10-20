use crate::media::media_db;

pub struct Ctx {
    pub media_db: Box<dyn media_db::MediaDb>,
}

impl Ctx {
    pub fn new() -> Ctx {
        let media_db = Box::new(media_db::random::Random::new());
        let ctx = Ctx { media_db };
        ctx
    }
}
