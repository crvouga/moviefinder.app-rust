use crate::core::html::*;

pub struct Props {
    pub id: String,
    pub label: String,
    pub name: String,
    pub checked: bool,
    pub disabled: bool,
}

pub fn view(props: Props, attrs: &[Attr]) -> Elem {
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
                    checked(props.checked),
                    disabled(props.disabled),
                ],
                &[],
            ),
            label(
                &[
                    class("flex items-center justify-center rounded-full w-fit border border-neutral-600 bg-neutral-800 px-2 py-1 text-sm text-white peer-checked:bg-white font-bold peer-checked:font-bold peer-checked:text-black enabled:active:opacity-80 cursor-pointer select-none"),
                    for_(&props.id),
                ],
                &[text(&props.label)],
            ),
        ],
    )
}
