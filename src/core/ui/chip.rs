use crate::core::html::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Props {
    pub id: String,
    pub label: String,
    pub name: String,
    pub checked: bool,
    pub disabled: bool,
    pub size: Size,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Size {
    Small,
    Medium,
    Large,
}

impl Default for Size {
    fn default() -> Self {
        Size::Medium
    }
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
                    class_list(&[
                        "flex items-center justify-center rounded-full font-bold w-fit bg-neutral-800 text-white cursor-pointer select-none",
                        "peer-checked:bg-white peer-checked:font-bold peer-checked:text-black enabled:active:opacity-80",
                        match props.size {
                            Size::Small => "text-xs px-2 py-1",
                            Size::Medium => "text-sm px-2.5 py-1.5",
                            Size::Large => "text-base px-3 py-2",
                        },
                    ]),
                    for_(&props.id),
                ],
                &[text(&props.label)],
            ),
        ],
    )
}
