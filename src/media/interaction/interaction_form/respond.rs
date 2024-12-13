use crate::{
    core::{
        html::{button, children::text, div, span, style, unsafe_text, Elem},
        ui::icon_button::IconButton,
    },
    media::interaction::interaction_name::InteractionName,
};

pub fn view_form_bottom_bar() -> Elem {
    div().class("flex flex-row items-center").children(vec![
        // InteractionName::Liked.view_icon_button(),
        // InteractionName::Disliked.view_icon_button(),
        // InteractionName::Interested.view_icon_button(),
        // InteractionName::NotInterested.view_icon_button(),
        InteractionName::Seen.view_bottom_button(),
        InteractionName::NotSeen.view_bottom_button(),
    ])
}

impl InteractionName {
    fn view_bottom_button(self) -> Elem {
        button()
            .class("flex flex-row p-4 flex-1 gap-1 items-center justify-center active:opacity-80")
            .child(self.view_icon(false, "size-8 shadow"))
            .child(
                span()
                    .class("text-sm font-bold shadow")
                    .child(text(&self.to_name())),
            )
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
