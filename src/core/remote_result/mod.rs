use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RemoteResult<T, E> {
    Loading,
    Ok(T),
    Err(E),
}

impl<T, E> From<Result<T, E>> for RemoteResult<T, E> {
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(ok) => RemoteResult::Ok(ok),
            Err(err) => RemoteResult::Err(err),
        }
    }
}

impl<T, E: Default> From<RemoteResult<T, E>> for Result<T, E> {
    fn from(result: RemoteResult<T, E>) -> Self {
        match result {
            RemoteResult::Loading => Result::Err(E::default()),
            RemoteResult::Ok(ok) => Result::Ok(ok),
            RemoteResult::Err(err) => Result::Err(err),
        }
    }
}
