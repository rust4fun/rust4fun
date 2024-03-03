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

#[cfg(test)]
pub mod test_utils {
    use crate::DbConnector;
    use sqlx::PgPool;

    pub const TEST_SECRET: &str = "secret";

    pub fn test_db_connector(pool: PgPool) -> DbConnector {
        DbConnector::new(pool, TEST_SECRET.into())
    }
}
