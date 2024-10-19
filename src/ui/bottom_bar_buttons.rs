use crate::html::*;

pub struct Button {
    pub text: String,
    pub href: String,
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
                a(
                    &[
                        href(&btn.href),
                        class(
                            "flex flex-1 items-center justify-center gap-0.5 flex-col text-sm py-2.5",
                        ),
                    ],
                    &[btn.icon.clone(), text(&btn.text)],
                )
            })
            .collect::<Vec<Elem>>(),
    )
}
