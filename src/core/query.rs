use super::pagination::Pagination;

#[derive(Debug, Clone)]
pub struct Query<F> {
    pub limit: usize,
    pub offset: usize,
    pub filter: Filter<F>,
}

impl<F> From<&Query<F>> for Pagination {
    fn from(query: &Query<F>) -> Pagination {
        Pagination {
            limit: query.limit,
            offset: query.offset,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Op {
    Eq,
}

#[derive(Debug, Clone)]
pub enum Filter<F> {
    Clause(F, Op, String),
    And(Vec<Filter<F>>),
    Or(Vec<Filter<F>>),
    None,
}

impl<T> Filter<T> {
    pub fn clause(field: T, operator: Op, value: String) -> Filter<T> {
        Filter::Clause(field, operator, value)
    }
}
