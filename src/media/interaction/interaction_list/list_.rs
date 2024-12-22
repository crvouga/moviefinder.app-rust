use crate::{
    list::list::List, media::interaction::interaction_name::InteractionName, user::user_id::UserId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaInteractionList {
    pub list: List,
    pub interaction_name: InteractionName,
    pub user_id: UserId,
}

impl From<MediaInteractionList> for List {
    fn from(media_interaction_list: MediaInteractionList) -> Self {
        media_interaction_list.list
    }
}
