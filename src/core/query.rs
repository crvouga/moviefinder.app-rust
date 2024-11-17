use super::pagination::{PageBased, Pagination};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Query<QueryField> {
    pub limit: usize,
    pub offset: usize,
    pub filter: QueryFilter<QueryField>,
}

impl<Field> From<&Query<Field>> for Pagination {
    fn from(query: &Query<Field>) -> Pagination {
        Pagination {
            limit: query.limit,
            offset: query.offset,
        }
    }
}

impl<Field> From<(&Query<Field>, usize)> for PageBased {
    fn from((query, page_size): (&Query<Field>, usize)) -> PageBased {
        let pagination: Pagination = query.into();
        let page_based = PageBased::from((pagination, page_size));
        page_based
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum QueryFilter<QueryField> {
    Clause(QueryField, QueryOp, String),
    And(Vec<QueryFilter<QueryField>>),
    Or(Vec<QueryFilter<QueryField>>),
    #[default]
    None,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QueryOp {
    Eq,
}
