use super::route::Route;
use crate::info;
use crate::{core::http::response_writer::ResponseWriter, ctx::Ctx, req::Req};
use crate::{
    core::{
        html::{button, children::text, div, span, style, unsafe_text, Elem},
        ui::icon_button::IconButton,
    },
    media::interaction::interaction_name::InteractionName,
};

pub async fn respond(
    ctx: &Ctx,
    _r: &Req,
    route: &Route,
    _w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::Record(interaction) => {
            info!(ctx.logger, "interaction: {:?}", interaction);
            Ok(())
        }
    }
}

pub fn view_form_bottom_bar() -> Elem {
    div().class("flex flex-row items-center").children(vec![
        InteractionName::Seen.view_bottom_button(false),
        InteractionName::NotSeen.view_bottom_button(false),
        // InteractionName::Liked.view_bottom_button(false),
        // InteractionName::Disliked.view_bottom_button(false),
        // InteractionName::Interested.view_bottom_button(false),
        // InteractionName::NotInterested.view_bottom_button(false),
    ])
}

impl InteractionName {
    fn view_bottom_button(self, selected: bool) -> Elem {
        button()
            .class("flex flex-row h-16 flex-1 gap-1 items-center justify-center active:opacity-80 text-base font-bold")
            .data_class(|b| b.class("text-blue-500", if selected { "true" } else { "false" }))
            .child(self.view_icon(selected, "size-7"))
            .child(text(&self.to_name()))
    }
}

pub fn _view_form_icon_buttons_vertical() -> Elem {
    div()
        .class("flex flex-col pb-2")
        .children(vec![
            InteractionName::Liked._view_icon_button(),
            InteractionName::Disliked._view_icon_button(),
            InteractionName::Interested._view_icon_button(),
            InteractionName::NotInterested._view_icon_button(),
            InteractionName::Seen._view_icon_button(),
            InteractionName::NotSeen._view_icon_button(),
        ])
        .child(style().child(unsafe_text(
            r#"
        .shadow { 
            filter: drop-shadow(0px 0px 6px black) drop-shadow(0px 0px 4px black); 
            text-shadow: 0px 0px 6px black, 0px 0px 4px black; 
        }
        "#,
        )))
}

impl InteractionName {
    fn _view_icon_button(self) -> Elem {
        let cloned = self.clone();
        IconButton::default()
            .label(self.to_name())
            .icon(move |_c| cloned.view_icon(true, "size-10 shadow"))
            .view()
            .class("flex flex-col gap-0.5 p-1.5")
            .child(
                span()
                    .class("text-xs font-bold shadow")
                    .child(text(&self.to_name())),
            )
    }
}
