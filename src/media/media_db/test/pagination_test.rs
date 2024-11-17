// test/limit_offset_test.rs
#[cfg(test)]
mod limit_offset_test {
    use super::super::fixture::fixtures;
    use crate::core::query::{QueryFilter, Query};

    #[tokio::test]
    async fn test_limit_and_offset() {
        for f in fixtures().await {
            let limit: usize = 40;
            let query = Query {
                limit,
                offset: 0,
                r#where: QueryFilter::None,
            };
            let result = f.media_db.query(query).await.unwrap();
            assert_eq!(result.items.len(), limit);
            assert_eq!(result.limit, limit);
            assert_eq!(result.offset, 0);
        }
    }

    #[tokio::test]
    async fn test_offset() {
        for f in fixtures().await {
            let limit: usize = 4;
            let offset: usize = 20;
            let queried = f
                .media_db
                .query(Query {
                    limit,
                    offset,
                    r#where: QueryFilter::None,
                })
                .await
                .unwrap();
            assert_eq!(queried.items.len(), limit);
        }
    }

    #[tokio::test]
    async fn test_offset_40() {
        for f in fixtures().await {
            let limit: usize = 4;
            let offset: usize = 40;
            let queried = f
                .media_db
                .query(Query {
                    limit,
                    offset,
                    r#where: QueryFilter::None,
                })
                .await
                .unwrap();
            assert_eq!(queried.items.len(), limit);
        }
    }
}
