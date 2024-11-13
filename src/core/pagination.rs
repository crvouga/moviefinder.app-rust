#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Pagination {
    pub limit: usize,
    pub offset: usize,
}

#[derive(Debug, Clone)]
pub struct Paginated<T> {
    pub items: Vec<T>,
    pub total: usize,
    #[allow(dead_code)]
    pub limit: usize,
    #[allow(dead_code)]
    pub offset: usize,
}

impl<T> Default for Paginated<T> {
    fn default() -> Self {
        Self {
            items: vec![],
            total: 0,
            limit: 0,
            offset: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PageBased {
    pub start_page: usize,
    pub end_page: usize,
    pub page_size: usize,
    pub index: usize,
}

impl From<(Pagination, usize)> for PageBased {
    fn from((pagination, page_size): (Pagination, usize)) -> Self {
        let page_count = (pagination.limit as f64 / page_size as f64).ceil() as usize;
        let start_page = (pagination.offset / page_size) + 1;
        let index = pagination.offset % page_size;
        let end_page_offset = if index == 0 { 0 } else { 1 };
        let end_page = start_page + page_count - 1 + end_page_offset;

        PageBased {
            start_page,
            end_page,
            page_size,
            index,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_page() {
        let pagination = Pagination {
            limit: 20,
            offset: 0,
        };
        let page_size = 20;
        let expected = PageBased {
            index: 0,
            page_size,
            start_page: 1,
            end_page: 1,
        };
        let actual: PageBased = (pagination, page_size).into();
        assert_eq!(actual.start_page, expected.start_page);
        assert_eq!(actual.end_page, expected.end_page);
        assert_eq!(actual.index, expected.index);
        assert_eq!(actual.page_size, expected.page_size);
    }

    #[test]
    fn second_page() {
        let pagination = Pagination {
            limit: 20,
            offset: 20,
        };
        let page_size = 20;
        let expected = PageBased {
            index: 0,
            page_size,
            start_page: 2,
            end_page: 2,
        };
        let actual: PageBased = (pagination, page_size).into();
        assert_eq!(actual.start_page, expected.start_page);
        assert_eq!(actual.end_page, expected.end_page);
        assert_eq!(actual.index, expected.index);
        assert_eq!(actual.page_size, expected.page_size);
    }

    #[test]
    fn third_page() {
        let pagination = Pagination {
            limit: 20,
            offset: 40,
        };
        let page_size = 20;
        let expected = PageBased {
            index: 0,
            page_size,
            start_page: 3,
            end_page: 3,
        };
        let actual: PageBased = (pagination, page_size).into();
        assert_eq!(actual.start_page, expected.start_page);
        assert_eq!(actual.end_page, expected.end_page);
        assert_eq!(actual.index, expected.index);
        assert_eq!(actual.page_size, expected.page_size);
    }

    #[test]
    fn first_two_pages() {
        let pagination = Pagination {
            limit: 40,
            offset: 0,
        };
        let page_size = 20;
        let expected = PageBased {
            index: 0,
            page_size,
            start_page: 1,
            end_page: 2,
        };
        let actual: PageBased = (pagination, page_size).into();
        assert_eq!(actual.start_page, expected.start_page);
        assert_eq!(actual.end_page, expected.end_page);
        assert_eq!(actual.index, expected.index);
        assert_eq!(actual.page_size, expected.page_size);
    }

    #[test]
    fn second_and_third_pages() {
        let pagination = Pagination {
            limit: 40,
            offset: 20,
        };
        let page_size = 20;
        let expected = PageBased {
            index: 0,
            page_size,
            start_page: 2,
            end_page: 3,
        };
        let actual: PageBased = (pagination, page_size).into();
        assert_eq!(actual.start_page, expected.start_page);
        assert_eq!(actual.end_page, expected.end_page);
        assert_eq!(actual.index, expected.index);
        assert_eq!(actual.page_size, expected.page_size);
    }

    #[test]
    fn second_page_with_fraction() {
        let pagination = Pagination {
            offset: (20.0_f64 * 1.5).floor() as usize,
            limit: (20.0_f64 / 2.0).floor() as usize,
        };
        let page_size = 20;
        let expected = PageBased {
            index: (20.0_f64 / 2.0).floor() as usize,
            page_size,
            start_page: 2,
            end_page: 3,
        };
        let actual: PageBased = (pagination, page_size).into();
        assert_eq!(actual.start_page, expected.start_page);
        assert_eq!(actual.end_page, expected.end_page);
        assert_eq!(actual.index, expected.index);
        assert_eq!(actual.page_size, expected.page_size);
    }

    #[test]
    fn edge_case() {
        let pagination = Pagination {
            limit: 10,
            offset: 19,
        };
        let page_size = 20;
        let expected = PageBased {
            index: 19,
            start_page: 1,
            end_page: 2,
            page_size,
        };
        let actual: PageBased = (pagination, page_size).into();
        assert_eq!(actual.start_page, expected.start_page);
        assert_eq!(actual.end_page, expected.end_page);
        assert_eq!(actual.index, expected.index);
        assert_eq!(actual.page_size, expected.page_size);
    }
}
