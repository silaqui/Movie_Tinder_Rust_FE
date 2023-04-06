use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Movie{
    pub id: String,
    pub title: String,
    pub genres: Vec<String>,
    pub description: String,
    pub poster_url: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vote{
    pub result : String,
    pub movie_id : String,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct VoteResult{
    pub is_match: bool,
    pub movie: Movie,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct Session {
    pub session_id : String,
    pub is_match: bool,
    pub movie: Movie,
}