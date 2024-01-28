use crate::types::{ArticleId, UserId};
use crate::DbConnector;
use chrono::NaiveDateTime;
use derive_new::new;
use std::sync::Arc;

#[derive(sqlx::FromRow)]
pub struct ArticleEntity {
    pub id: ArticleId,
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub image_url: String,
    pub registered_by: UserId,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(new)]
pub struct InputArticleEntity {
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub image_url: String,
    pub registered_by: UserId,
}

pub struct ArticleRepository(Arc<DbConnector>);

impl ArticleRepository {
    pub fn new(db: Arc<DbConnector>) -> Self {
        Self(db)
    }

    pub async fn create(&self, id: ArticleId, input: InputArticleEntity) {
        let pool = self.0.get_pool();

        let res = sqlx::query!(
            r#"
                INSERT INTO articles
                    (id, url, title, description, image_url, registered_by)
                values
                    ($1::UUID, $2, $3, $4, $5, $6::UUID)
            "#,
            id.id(),
            input.title,
            input.url,
            input.description,
            input.image_url,
            input.registered_by.id()
        )
        .execute(&pool)
        .await;

        tracing::info!("{res:?}");
    }

    pub async fn find_by_id(&self, id: ArticleId) -> ArticleEntity {
        let pool = self.0.get_pool();

        let article = sqlx::query_as!(
            ArticleEntity,
            r#"
                SELECT *
                FROM articles
                WHERE id = $1::UUID
            "#,
            id.id()
        )
        .fetch_one(&pool)
        .await;

        article.unwrap()
    }

    pub async fn list_by_user(&self, user_id: UserId) -> Vec<ArticleEntity> {
        let pool = self.0.get_pool();

        let article = sqlx::query_as!(
            ArticleEntity,
            r#"
                SELECT *
                FROM articles
                WHERE registered_by = $1::UUID
            "#,
            user_id.id()
        )
        .fetch_all(&pool)
        .await;

        article.unwrap()
    }
}
