use rust_study_shared as shared;

use crate::error::Error;
use crate::DbConnector;
use derive_new::new;
use shared::UserId;
use std::sync::Arc;

#[derive(Debug)]
pub struct UserEntity {
    pub id: UserId,
    pub name: Option<String>,
}

#[derive(Debug, new)]
pub struct InputUserEntity {
    pub name: Option<String>,
    pub email: String,
    pub password: String,
}

#[derive(Debug, new)]
pub struct InputUserValidateEntity {
    pub email: String,
    pub password: String,
}

pub struct UserRepository(Arc<DbConnector>);

impl UserRepository {
    pub fn new(db: Arc<DbConnector>) -> Self {
        Self(db)
    }

    pub async fn create(&self, id: UserId, input: InputUserEntity) -> Result<(), Error> {
        let pool = self.0.get_pool();

        let res = sqlx::query!(
            r#"
                INSERT INTO users
                    (id, name, email, password)
                values
                    ($1::UUID, $2, digest($3, 'sha256'), pgp_sym_encrypt_bytea($4, $5))
                    ON CONFLICT DO NOTHING
            "#,
            id.id(),
            input.name,
            input.email,
            input.password.as_bytes(),
            self.0.get_secret()
        )
        .execute(&pool)
        .await
        .map_err(Error::Database)?;

        if res.rows_affected() == 0 {
            return Err(Error::AlreadyExsited("articles".into()));
        }

        Ok(())
    }

    pub async fn validate_and_get(
        &self,
        input: InputUserValidateEntity,
    ) -> Result<UserEntity, Error> {
        let pool = self.0.get_pool();

        let user = sqlx::query_as!(
            UserEntity,
            r#"
            SELECT 
                id
                , name
            FROM users
            WHERE 
                email = digest($1, 'sha256')
            AND
            pgp_sym_decrypt_bytea(password, $3) = $2
        "#,
            input.email,
            input.password.as_bytes(),
            self.0.get_secret()
        )
        .fetch_one(&pool)
        .await
        .map_err(Error::Database)?;

        Ok(user)
    }

    pub async fn find_by_id(&self, id: UserId) -> Result<UserEntity, Error> {
        let pool = self.0.get_pool();

        let user = sqlx::query_as!(
            UserEntity,
            r#"
            SELECT 
                id
                , name
            FROM users
            WHERE id = $1::UUID
        "#,
            id.id()
        )
        .fetch_one(&pool)
        .await
        .map_err(Error::Database)?;

        Ok(user)
    }
}

#[cfg(test)]
pub mod test_utils {
    use super::*;

    use crate::repository::test_utils::TEST_SECRET;
    use anyhow::Result;
    use sqlx::PgPool;

    pub async fn create_user(pool: PgPool) -> Result<UserId> {
        let id = UserId::new_v4();
        let name = "dummy_user";
        let email = "dummy@example.com";
        let password = "dummy_password";

        let _ = sqlx::query!(
            r#"
                INSERT INTO users
                    (id, name, email, password)
                values
                    ($1::UUID, $2, digest($3, 'sha256'), pgp_sym_encrypt_bytea($4, $5))
                    ON CONFLICT DO NOTHING
            "#,
            id.clone().id(),
            name,
            email,
            password.as_bytes(),
            TEST_SECRET
        )
        .execute(&pool)
        .await?;

        Ok(id)
    }
}
