use derive_new::new;
use serde::Serialize;

#[derive(Debug, Serialize, new)]
pub struct ListResponse<T> {
    pub items: Vec<T>,
    pub offset: i64,
    pub limit: i64,
    pub total: u64,
}
