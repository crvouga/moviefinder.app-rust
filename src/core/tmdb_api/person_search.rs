// https://developer.themoviedb.org/reference/search-person
use super::{person::PersonResult, TmdbApi};
use crate::core::http::query_params::QueryParams;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PersonSearchResponse {
    pub page: Option<usize>,
    pub results: Option<Vec<PersonResult>>,
    pub total_pages: Option<usize>,
    pub total_results: Option<usize>,
}

impl TmdbApi {
    /// Searches for persons in the TMDB API.
    ///
    /// # Arguments
    ///
    /// * `query` - The search query string.
    /// * `page` - The page number to fetch (optional).
    ///
    /// # Returns
    ///
    /// Returns a `PersonSearchResponse` on success or a `String` error message.
    pub async fn person_search(
        self: &TmdbApi,
        query: &str,
        page: &usize,
    ) -> Result<PersonSearchResponse, String> {
        let mut params = QueryParams::empty();
        params.insert("query", query.to_string());

        params.insert("page", page.to_string());

        let req = self.to_get_request("/3/search/person", params);

        println!("req: {:?}", req);

        let sent = self.http_client.send(req).await;

        println!("sent: {:?}", sent);

        let response = sent.map_err(|err| err.to_string())?;

        let parsed = serde_json::from_str::<PersonSearchResponse>(&response.body)
            .map_err(|err| format!("Error parsing response: {} {}", err, response.body))?;

        Ok(parsed)
    }
}
