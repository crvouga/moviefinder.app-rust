// test/deduplicated_test.rs
#[cfg(test)]
mod deduplicated_test {
    use super::super::fixture::{fixtures, frequencies};
    use crate::core::query::{Query, QueryFilter};
    use crate::media::media_id::MediaId;
    use std::collections::HashSet;

    #[tokio::test]
    async fn test_no_duplicates() {
        for f in fixtures().await {
            let limit: usize = 50;
            let queried = f
                .media_db
                .query(Query {
                    limit,
                    offset: 0,
                    filter: QueryFilter::None,
                })
                .await
                .unwrap();
            let media_ids: Vec<MediaId> = queried
                .items
                .iter()
                .map(|media| media.media_id.clone())
                .collect();
            let media_id_frequencies = frequencies(media_ids.clone());

            let duplicate_media_ids: Vec<MediaId> = media_ids
                .clone()
                .into_iter()
                .filter(|media_id| media_id_frequencies.get(media_id).unwrap_or(&0) > &1)
                .collect();
            let unique_media_ids: HashSet<MediaId> = media_ids.clone().into_iter().collect();

            assert_eq!(duplicate_media_ids.len(), 0);
            assert_eq!(media_ids.len(), unique_media_ids.len());
        }
    }

    #[tokio::test]
    async fn test_no_duplicates_small_limit_and_offset() {
        for f in fixtures().await {
            let limit: usize = 4;
            let queried = f
                .media_db
                .query(Query {
                    limit,
                    offset: 5,
                    filter: QueryFilter::None,
                })
                .await
                .unwrap();
            let media_ids = queried
                .items
                .iter()
                .map(|media| media.media_id.clone())
                .collect::<Vec<MediaId>>();
            let media_id_frequencies = frequencies(media_ids.clone());

            let duplicate_media_ids = media_ids
                .iter()
                .filter(|media_id| media_id_frequencies.get(media_id).unwrap_or(&0) > &1)
                .collect::<Vec<&MediaId>>();
            let unique_media_ids = media_ids.iter().collect::<HashSet<&MediaId>>();

            assert_eq!(duplicate_media_ids.len(), 0);
            assert_eq!(media_ids.len(), unique_media_ids.len());
        }
    }
}
