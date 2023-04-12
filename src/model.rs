use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq, Deserialize)]
pub struct Movie {
    pub id: String,
    pub title: String,
    pub genres: Vec<String>,
    pub description: String,
    pub poster_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum VoteResult {
    WATCH,
    SKIP,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoteDTO {
    pub result: VoteResult,
    pub movie_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionStateDTO {
    pub session_id: Option<usize>,
    pub match_movie: Option<Movie>,
    pub next_movie: Option<Movie>,
}