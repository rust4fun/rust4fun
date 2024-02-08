use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Pagenations {
    pub offset: i64,
    pub limit: i64,
}
