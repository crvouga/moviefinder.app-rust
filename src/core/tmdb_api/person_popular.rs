use super::{person::PersonResult, TmdbApi};
use crate::core::http::query_params::QueryParams;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PopularPersonResponse {
    pub page: Option<usize>,
    pub results: Option<Vec<PersonResult>>,
    pub total_pages: Option<usize>,
    pub total_results: Option<usize>,
}

impl TmdbApi {
    /// Fetches the list of popular people from the TMDB API.
    ///
    /// # Arguments
    ///
    /// * `page` - The page number to fetch (optional).
    ///
    /// # Returns
    ///
    /// Returns a `PopularPersonResponse` on success or a `String` error message.
    pub async fn person_popular(
        self: &TmdbApi,
        page: usize,
    ) -> Result<PopularPersonResponse, String> {
        let mut params = QueryParams::empty();

        params.insert("page", page.to_string());

        let req = self.to_get_request("/3/person/popular", params);

        let sent = self.http_client.send(req).await;

        let response = sent.map_err(|err| err.to_string())?;

        let parsed = serde_json::from_str::<PopularPersonResponse>(&response.body)
            .map_err(|err| format!("Error parsing response: {} {}", err, response.body))?;

        Ok(parsed)
    }
}
