use super::pagination::{PageBased, Pagination};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Query<Field> {
    pub limit: usize,
    pub offset: usize,
    pub filter: Filter<Field>,
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
pub enum Filter<Field> {
    Clause(Field, Op, String),
    And(Vec<Filter<Field>>),
    Or(Vec<Filter<Field>>),
    #[default]
    None,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Op {
    Eq,
}
