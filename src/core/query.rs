use super::pagination::Pagination;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum Filter<F> {
    Clause(F, Op, String),
    And(Vec<Filter<F>>),
    Or(Vec<Filter<F>>),
    #[default]
    None,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Op {
    Eq,
}

impl<T> Filter<T> {
    pub fn clause(field: T, operator: Op, value: String) -> Filter<T> {
        Filter::Clause(field, operator, value)
    }
}
