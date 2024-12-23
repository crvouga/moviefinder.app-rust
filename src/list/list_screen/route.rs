use crate::list::list::List;

#[allow(dead_code)]
pub enum Route<TList: List + Clone + 'static> {
    Screen { list: TList },
    IntersectedBottom { list: TList },
}
