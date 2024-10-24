#[allow(dead_code)]
pub struct Pagination {
    pub limit: u32,
    pub offset: u32,
}

#[derive(Debug, Clone)]
pub struct Paginated<T> {
    pub items: Vec<T>,
    pub total: u32,
    #[allow(dead_code)]
    pub limit: u32,
    #[allow(dead_code)]
    pub offset: u32,
}

impl<T> Paginated<T> {
    #[allow(dead_code)]
    pub fn empty() -> Self {
        Self {
            items: vec![],
            total: 0,
            limit: 0,
            offset: 0,
        }
    }
}
