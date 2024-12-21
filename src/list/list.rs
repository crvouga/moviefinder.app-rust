use super::{list_id::ListId, list_variant::ListVariant};
use crate::{core::posix::Posix, user::user_id::UserId};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct List {
    pub id: ListId,
    pub created_at_posix: Posix,
    pub updated_at_posix: Posix,
    pub deleted_at_posix: Option<Posix>,
    pub name: String,
    pub description: String,
    pub creator_user_id: UserId,
    pub variant: ListVariant,
}
