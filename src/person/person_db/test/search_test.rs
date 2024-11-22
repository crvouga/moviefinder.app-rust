#[cfg(test)]
mod search_test {
    use super::super::fixture::fixtures;
    use crate::core::query::{Query, QueryFilter};

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
}
