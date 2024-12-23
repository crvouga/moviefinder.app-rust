use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RemoteResult<TOk, TErr> {
    Loading,
    Ok(TOk),
    Err(TErr),
}

impl<TOk, TErr> From<Result<TOk, TErr>> for RemoteResult<TOk, TErr> {
    fn from(result: Result<TOk, TErr>) -> Self {
        match result {
            Ok(ok) => RemoteResult::Ok(ok),
            Err(err) => RemoteResult::Err(err),
        }
    }
}

impl<TOk, TErr: Default> From<RemoteResult<TOk, TErr>> for Result<TOk, TErr> {
    fn from(result: RemoteResult<TOk, TErr>) -> Self {
        match result {
            RemoteResult::Loading => Result::Err(TErr::default()),
            RemoteResult::Ok(ok) => Result::Ok(ok),
            RemoteResult::Err(err) => Result::Err(err),
        }
    }
}
