#[cfg(test)]
mod search_test {
    use super::super::fixture::fixtures;
    use crate::{
        core::query::{Query, QueryFilter, QueryOp},
        person::person_db::interface::PersonQueryField,
    };

    #[tokio::test]
    async fn test_search() {
        for f in fixtures().await {
            let limit: usize = 50;
            let queried = f
                .person_db
                .query(Query {
                    limit,
                    offset: 0,
                    filter: QueryFilter::None,
                })
                .await
                .unwrap();

            assert!(queried.items.len() > 0);
        }
    }

    #[tokio::test]
    async fn test_search_by_name() {
        for f in fixtures().await {
            let limit: usize = 50;
            let queried = f
                .person_db
                .query(Query {
                    limit,
                    offset: 0,
                    filter: QueryFilter::Clause(
                        PersonQueryField::Name,
                        QueryOp::Like,
                        "tom cruise".to_string(),
                    ),
                })
                .await
                .unwrap();

            assert!(queried.items.len() > 0);
        }
    }
}
