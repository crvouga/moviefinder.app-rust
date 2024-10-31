use super::pagination::{PageBased, Pagination};
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

impl<F> From<(&Query<F>, usize)> for PageBased {
    fn from((query, page_size): (&Query<F>, usize)) -> PageBased {
        let pagination: Pagination = query.into();
        let page_based = PageBased::from((pagination, page_size));
        page_based
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
