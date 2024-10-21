pub struct Query<F> {
    pub limit: u32,
    pub offset: u32,
    pub filter: Filter<F>,
}

pub enum Operator {
    Eq,
}

pub enum Filter<F> {
    Clause(Clause<F>),
    And(Vec<Filter<F>>),
    Or(Vec<Filter<F>>),
    None,
}

pub struct Clause<F> {
    pub field: F,
    pub operator: Operator,
    pub value: String,
}

impl<T> Filter<T> {
    pub fn empty<F>() -> Filter<F> {
        Filter::And(vec![])
    }

    pub fn clause(field: T, operator: Operator, value: String) -> Filter<T> {
        Filter::Clause(Clause {
            field,
            operator,
            value,
        })
    }
}
