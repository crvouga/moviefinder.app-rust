use std::fmt::Debug;

use crate::{
    core::{
        html::{div, p, Elem},
        http::response_writer::ResponseWriter,
        pagination::Paginated,
        ui::top_bar::TopBar,
    },
    ctx::Ctx,
    list::{list::List, list_item::ListItem},
    req::Req,
};

use super::route::Route;

pub async fn respond<TList: List + Debug>(
    // list_screen_db: &impl ListScreenDb<TList>,
    _ctx: &Ctx,
    _r: &Req,
    route: &Route<TList>,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::IntersectedBottom { list } => {
            println!("IntersectedBottom: {:?}", list);
            Ok(())
        }
        Route::Screen { list, back_url } => {
            let model = ViewModel {
                back_url: Some(back_url.clone()),
                list: Some(list.clone()),
                list_items: None,
            };
            w.send_screen(model.view()).await?;
            // let found = list_screen_db.find_list_items(0, 10, list.to_owned()).await;
            Ok(())
        }
    }
}

#[derive(Clone, Default)]
pub struct ViewModel<TList: List> {
    pub list: Option<TList>,
    #[allow(dead_code)]
    pub list_items: Option<Paginated<ListItem>>,
    pub back_url: Option<String>,
}

impl<T: List> ViewModel<T> {
    pub fn view(self) -> Elem {
        let list = self.list.clone();
        let name = list.clone().map(|l| l.name());
        let art = list.map(|l| l.view_art("size-32 rounded shrink-0"));
        div()
            .class("w-full h-full flex flex-col")
            .child(
                TopBar::default()
                    .back_url(self.back_url.unwrap_or("".to_owned()))
                    .title(&name.clone().unwrap_or_default())
                    .view(),
            )
            .child(
                div()
                    .class("w-full flex items-center justify-center pt-12 flex-col gap-6 px-6")
                    .child(art.unwrap_or_default())
                    .child(
                        p().class("w-full text-center text-3xl font-bold")
                            .child_text(&name.unwrap_or_default()),
                    ),
            )
            .child(div().id("list-items").class("w-full flex flex-col gap-4"))
    }
}
