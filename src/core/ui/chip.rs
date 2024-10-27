use crate::core::html::*;

pub struct ChipProps {
    pub id: String,
    pub label: String,
    pub name: String,
    pub initial_state: bool,
}

pub fn view(props: ChipProps, attrs: &[Attr]) -> Elem {
    div(
        attrs,
        &[
            input(
                &[
                    class("hidden peer"), 
                    type_("checkbox"), 
                    id(&props.id),
                    name(&props.name),
                    value(&props.id),
                    checked(props.initial_state),
                ],
                &[],
            ),
            label(
                &[
                    class("flex items-center justify-center rounded-full w-fit border border-neutral-600 bg-neutral-800 px-4 py-2 text-white peer-checked:bg-white font-bold peer-checked:font-bold peer-checked:text-black active:opacity-80 cursor-pointer select-none"),
                    for_(&props.id),
                ],
                &[text(&props.label)],
            ),
        ],
    )
}
