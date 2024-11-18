// test/filter_by_genre_id_test.rs
#[cfg(test)]
mod filter_by_genre_id_test {
    use super::super::fixture::fixtures;
    use crate::{
        core::query::{Query, QueryFilter, QueryOp},
        media::media_db::interface::MediaQueryField,
    };

    #[tokio::test]
    async fn test_filter_by_genre_id() {
        for f in fixtures().await {
            let genre_id = f.random_genre_id().await;
            let queried = f
                .media_db
                .query(Query {
                    limit: 10,
                    offset: 0,
                    r#where: QueryFilter::Clause(
                        MediaQueryField::GenreId,
                        QueryOp::Eq,
                        genre_id.to_string(),
                    ),
                })
                .await
                .unwrap();

            assert!(queried.items.len() > 0);
            assert!(queried
                .items
                .into_iter()
                .all(|item| item.media_genre_ids.contains(&genre_id)));
        }
    }

    #[tokio::test]
    async fn test_filter_and_two_genre_ids() {
        for f in fixtures().await {
            let genre_id_a = f.random_genre_id().await;
            let genre_id_b = f.random_genre_id().await;
            let queried = f
                .media_db
                .query(Query {
                    limit: 10,
                    offset: 0,
                    r#where: QueryFilter::And(vec![
                        QueryFilter::Clause(
                            MediaQueryField::GenreId,
                            QueryOp::Eq,
                            genre_id_a.to_string(),
                        ),
                        QueryFilter::Clause(
                            MediaQueryField::GenreId,
                            QueryOp::Eq,
                            genre_id_b.to_string(),
                        ),
                    ]),
                })
                .await
                .unwrap();

            assert!(queried.items.len() > 0);
            assert!(queried
                .items
                .into_iter()
                .all(|item| item.media_genre_ids.contains(&genre_id_a)
                    && item.media_genre_ids.contains(&genre_id_b)));
        }
    }

    #[tokio::test]
    async fn test_filter_or_two_genre_ids() {
        for f in fixtures().await {
            let genre_id_a = f.random_genre_id().await;
            let genre_id_b = f.random_genre_id().await;
            let queried = f
                .media_db
                .query(Query {
                    limit: 10,
                    offset: 0,
                    r#where: QueryFilter::Or(vec![
                        QueryFilter::Clause(
                            MediaQueryField::GenreId,
                            QueryOp::Eq,
                            genre_id_a.to_string(),
                        ),
                        QueryFilter::Clause(
                            MediaQueryField::GenreId,
                            QueryOp::Eq,
                            genre_id_b.to_string(),
                        ),
                    ]),
                })
                .await
                .unwrap();

            assert!(queried.items.len() > 0);
            assert!(queried
                .items
                .into_iter()
                .all(|item| item.media_genre_ids.contains(&genre_id_a)
                    || item.media_genre_ids.contains(&genre_id_b)));
        }
    }
}
