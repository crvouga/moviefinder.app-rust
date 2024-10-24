use super::{Request, Response};

pub async fn wrap_session_id<TSessionId, F, Fut>(handler: F) -> F
where
    F: Fn(Request) -> Fut + Send + Sync + 'static + Clone,
    Fut: std::future::Future<Output = Response> + Send + 'static,
{
    unimplemented!("Wrap session id")
}
