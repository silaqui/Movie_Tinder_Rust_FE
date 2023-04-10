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

#[derive(Debug, Serialize, Deserialize)]
pub struct NextMovie {
    pub session_id: String,
    pub is_match: bool,
    pub movie: Movie,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vote {
    pub result: VoteResult,
    pub movie_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum VoteResult {
    WATCH,
    SKIP,
}
