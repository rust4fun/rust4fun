pub mod articles;
pub mod planet_members;
pub mod planets;
pub mod spheres;
pub mod users;

use derive_new::new;

#[derive(Debug, new)]
pub struct Pagination {
    pub limit: i64,
    pub offset: i64,
}
