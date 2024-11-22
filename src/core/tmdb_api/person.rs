use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KnownFor {
    pub adult: Option<bool>,
    pub backdrop_path: Option<String>,
    pub id: Option<usize>,
    pub title: Option<String>,
    pub original_language: Option<String>,
    pub original_title: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub media_type: Option<String>,
    pub genre_ids: Option<Vec<usize>>,
    pub popularity: Option<f64>,
    pub release_date: Option<String>,
    pub video: Option<bool>,
    pub vote_average: Option<f64>,
    pub vote_count: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PersonResult {
    pub adult: Option<bool>,
    pub gender: Option<usize>,
    pub id: Option<usize>,
    pub known_for_department: Option<String>,
    pub name: Option<String>,
    pub original_name: Option<String>,
    pub popularity: Option<f64>,
    pub profile_path: Option<String>,
    pub known_for: Option<Vec<KnownFor>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetPersonResponse {
    pub page: Option<usize>,
    pub results: Option<Vec<PersonResult>>,
    pub total_pages: Option<usize>,
    pub total_results: Option<usize>,
}
