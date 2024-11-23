use crate::{ctx::Ctx, env::Env};

pub struct BaseFixture {
    pub ctx: Ctx,
    pub env: Env,
}

impl BaseFixture {
    pub async fn new() -> Self {
        let env = Env::load();

        let ctx = Ctx::new(&env).await.unwrap();

        Self { ctx, env }
    }
}
