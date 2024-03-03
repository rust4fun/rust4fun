use rust_study_db_connector as db;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Pagenations {
    pub offset: i64,
    pub limit: i64,
}

impl From<Pagenations> for db::Pagination {
    fn from(page: Pagenations) -> Self {
        db::Pagination::new(page.limit, page.offset)
    }
}
