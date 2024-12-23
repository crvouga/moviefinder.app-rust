// test/filter_by_id_test.rs
#[cfg(test)]
mod filter_by_id_test {
    use std::collections::HashSet;

    use super::super::fixture::fixtures;
    use crate::core::query::{Query, QueryFilter, QueryOp};
    use crate::media::media_db::interface::MediaQueryField;
    use crate::media::media_id::MediaId;

    #[tokio::test]
    async fn test_find_by_id() {
        for f in fixtures().await {
            let media_ids = vec![
                MediaId::new("123".to_string()),
                MediaId::new("124".to_string()),
                MediaId::new("125".to_string()),
            ];

            let query = Query {
                limit: media_ids.len().into(),
                offset: 0,
                filter: QueryFilter::Or(
                    media_ids
                        .iter()
                        .map(|id| {
                            QueryFilter::Clause(
                                MediaQueryField::MediaId,
                                QueryOp::Eq,
                                id.as_str().to_string(),
                            )
                        })
                        .collect(),
                ),
            };
            let result = f.media_db.query(query).await;
            let expected: HashSet<MediaId> = media_ids.iter().cloned().collect();
            let actual: HashSet<MediaId> =
                result.unwrap().items.into_iter().map(|m| m.id).collect();
            assert_eq!(expected, actual);
        }
    }
}
