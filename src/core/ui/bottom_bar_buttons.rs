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
            "flex items-center justify-center w-full border-t divide-x",
        )],
        &buttons
            .iter()
            .map(|btn| {
                a(&[
                    hx::get(&btn.hx_get),
                    hx::target(&btn.hx_target),
                    hx::Swap::InnerHtml.attr(),
                    hx::push_url(&btn.hx_get),
                    hx::Trigger::MouseDown.attr(),
                    hx::Preload::MouseDown.attr(),
                    class_list(
                        &[
                            "flex flex-1 items-center justify-center gap-0.5 flex-col text-sm py-2.5 cursor-pointer select-none active:opacity-75 transition-opacity",
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
