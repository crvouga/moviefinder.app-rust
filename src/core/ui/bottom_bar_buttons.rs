use crate::{core::html::*, core::hx};

pub struct Button {
    pub text: String,
    pub hx_get: String,
    pub hx_target: String,
    pub icon: Elem,
    pub active: bool,
}

pub fn view(buttons: &[Button]) -> Elem {
    div(
        &[class(
            "flex items-center justify-center w-full border-t divide-x h-16",
        )],
        &buttons
            .iter()
            .map(|btn| {
                a(&[
                    hx::get(&btn.hx_get),
                    hx::target(&btn.hx_target),
                    hx::Swap::InnerHtml.into(),
                    hx::push_url(&btn.hx_get),
                    hx::Trigger::Click.into(),
                    hx::Preload::MouseDown.into(),
                    class_list(
                        &[
                            "flex flex-1 items-center justify-center gap-0.5 flex-col text-sm h-full cursor-pointer select-none active:opacity-75 transition-opacity",
                            if btn.active {
                                "text-blue-500"
                            } else {
                                "text-white"
                            },
                        ]
                    ),
                ],
                &[btn.icon.clone(), text(&btn.text)],
                )
            })
            .collect::<Vec<Elem>>(),
    )
}
