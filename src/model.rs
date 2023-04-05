use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Movie{
    pub id: String,
    pub title: String,
    pub genres: Vec<String>,
    pub description: String,
    pub poster_url: String
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct NewSession{
    pub user_id: String,
    pub session_id: String,
}
