pub struct Pagination {
    pub limit: u32,
    pub offset: u32,
}

#[derive(Debug, Clone)]
pub struct Paginated<T> {
    pub items: Vec<T>,
    pub total: u32,
    pub limit: u32,
    pub offset: u32,
}

impl<T> Paginated<T> {
    pub fn empty() -> Self {
        Self {
            items: vec![],
            total: 0,
            limit: 0,
            offset: 0,
        }
    }
}
