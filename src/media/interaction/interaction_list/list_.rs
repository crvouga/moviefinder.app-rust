use serde::{Deserialize, Serialize};

use crate::{
    core::html::{div, Elem},
    list::{list::MediaList, list_id::MediaListId, list_item_id::MediaListItemId, list_screen},
    media::{interaction::interaction_name::InteractionName, media_id::MediaId},
    ui::route::AppRoute,
    user::{self, user_id::UserId},
};

use super::route::Route;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MediaInteractionList {
    pub interaction_name: InteractionName,
    pub user_id: UserId,
}

impl MediaList for MediaInteractionList {
    fn view_art(&self, class: &str) -> Elem {
        div()
            .class("bg-gradient-to-br from-[#D38ABF] via-[#434EA9] to-[#07413A]")
            .class(class)
            .child(
                div()
                    .class("w-full h-full p-4 flex flex-col items-center justify-center")
                    .child(self.interaction_name.view_icon(true, "w-full")),
            )
    }

    fn id(&self) -> MediaListId {
        let name_str = self.interaction_name.to_machine_string();
        let list_id_str = format!("interaction-list-{}-{}", name_str, self.user_id.as_str());
        let list_id = MediaListId::new(&list_id_str);
        list_id
    }

    fn name(&self) -> String {
        self.interaction_name.to_display_string()
    }

    fn details_url(&self) -> String {
        Route::ListScreen(list_screen::route::Route::Screen {
            back_url: user::route::Route::AccountScreen.url(),
            list: MediaInteractionList {
                user_id: self.user_id.clone(),
                interaction_name: self.interaction_name.clone(),
            },
        })
        .url()
    }
}

impl InteractionName {
    pub fn to_list_item_id(&self, list_id: MediaListId, media_id: MediaId) -> MediaListItemId {
        let name_str = self.to_machine_string();

        let list_item_id_str = format!(
            "interaction-list-item-{}-{}-{}",
            name_str,
            list_id.as_str(),
            media_id.as_str()
        );

        let list_item_id = MediaListItemId::from_string(&list_item_id_str);

        list_item_id
    }
}
