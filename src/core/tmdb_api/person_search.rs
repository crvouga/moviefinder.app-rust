// https://developer.themoviedb.org/reference/search-person
use super::{person::GetPersonResponse, TmdbApi};
use crate::core::{unstructured_data::UnstructuredData, url::query_params::QueryParams};

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
    ) -> Result<GetPersonResponse, String> {
        let params = QueryParams::empty()
            .insert("query", query.to_string())
            .insert("page", page.to_string());

        let req = self.to_get_request("/3/search/person", params);

        let sent = self.http_client.send(req).await;

        let response = sent.map_err(|err| err.to_string())?;

        let parsed = serde_json::from_str::<GetPersonResponse>(&response.clone().to_body_string())
            .map_err(|err| {
                format!(
                    "Error parsing response: {} {}",
                    err,
                    &response.to_body_string()
                )
            })?;

        Ok(parsed)
    }
}
