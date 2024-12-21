use crate::{
    list::{list_id::ListId, list_item_id::ListItemId},
    media::media_id::MediaId,
    user::user_id::UserId,
};

use super::InteractionName;

impl InteractionName {
    pub fn to_list_id(&self, user_id: UserId) -> ListId {
        let name_str = self.to_machine_string();
        let list_id_str = format!("interaction-list-{}-{}", name_str, user_id.as_str());
        let list_id = ListId::new(&list_id_str);
        list_id
    }

    pub fn to_list_name(&self) -> String {
        self.to_display_string()
    }

    pub fn to_list_description(&self) -> String {
        " ".to_string()
    }

    pub fn to_list_item_id(&self, list_id: ListId, media_id: MediaId) -> ListItemId {
        let name_str = self.to_machine_string();

        let list_item_id_str = format!(
            "interaction-list-item-{}-{}-{}",
            name_str,
            list_id.as_str(),
            media_id.as_str()
        );

        let list_item_id = ListItemId::from_string(&list_item_id_str);

        list_item_id
    }
}
