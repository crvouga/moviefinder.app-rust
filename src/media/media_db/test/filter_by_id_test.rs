// test/filter_by_id_test.rs
#[cfg(test)]
mod filter_by_id_test {
    use super::super::fixture::fixtures;
    use crate::core::query::{Filter, Op, Query};
    use crate::media::media_db::interface::MediaField;
    use crate::media::media_id::MediaId;

    #[tokio::test]
    async fn test_find_by_id() {
        for f in fixtures().await {
            let media_id = MediaId::new("123".to_string());
            let query = Query {
                limit: 1,
                offset: 0,
                filter: Filter::Clause(MediaField::MediaId, Op::Eq, media_id.as_str().to_string()),
            };
            let result = f.media_db.query(query).await;
            let first = result.unwrap().items.into_iter().next().unwrap();
            assert_eq!(first.media_id, media_id);
        }
    }
}
