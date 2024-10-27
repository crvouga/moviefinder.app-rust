use crate::core::html::*;

pub fn view(id_str: &str, label_str: &str) -> Elem {
    div(
        &[],
        &[
            input(
                &[
                    class("hidden peer"), 
                    type_("checkbox"), 
                    id(&id_str)
                ],
                &[],
            ),
            label(
                &[
                    class("flex items-center justify-center rounded-full w-fit border border-neutral-600 bg-neutral-800 px-4 py-2 text-white peer-checked:bg-white font-bold peer-checked:font-bold peer-checked:text-black active:opacity-80"),
                    for_(&id_str),
                ],
                &[text(label_str)],
            ),
        ],
    )
}
