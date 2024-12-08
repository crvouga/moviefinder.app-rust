#[cfg(test)]
use crate::{ctx::Ctx, env::Env};

#[cfg(test)]
pub struct BaseFixture {
    pub ctx: Ctx,
    pub env: Env,
}

#[cfg(test)]
impl BaseFixture {
    pub async fn new() -> Self {
        let env = Env::load();

        let ctx = Ctx::new(&env).await;

        Self { ctx, env }
    }
}
