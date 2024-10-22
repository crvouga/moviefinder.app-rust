#[derive(Debug, Clone)]
pub struct Query<F> {
    pub limit: u32,
    pub offset: u32,
    pub filter: Filter<F>,
}

#[derive(Debug, Clone)]
pub enum Op {
    Eq,
}

#[derive(Debug, Clone)]
pub enum Filter<F> {
    Clause(Clause<F>),
    And(Vec<Filter<F>>),
    Or(Vec<Filter<F>>),
    None,
}

#[derive(Debug, Clone)]
pub struct Clause<F> {
    pub field: F,
    pub operator: Op,
    pub value: String,
}

impl<T> Filter<T> {
    pub fn clause(field: T, operator: Op, value: String) -> Filter<T> {
        Filter::Clause(Clause {
            field,
            operator,
            value,
        })
    }
}
