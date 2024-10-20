pub struct Pagination {
    pub limit: u32,
    pub offset: u32,
}

pub struct Paginated<T> {
    pub items: Vec<T>,
    pub total: u32,
    pub limit: u32,
    pub offset: u32,
}
