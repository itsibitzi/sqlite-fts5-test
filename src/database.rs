use std::str::FromStr;

use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn open() -> anyhow::Result<Self> {
        let options = SqliteConnectOptions::from_str("sqlite://test.db")?
            .pragma("key", "'This is a password'")
            .create_if_missing(true);

        let pool = SqlitePool::connect_with(options).await?;

        sqlx::migrate!().run(&pool).await?;

        Ok(Self { pool })
    }

    pub async fn insert_post(&self, title: &str, body: &str) -> anyhow::Result<()> {
        let mut conn = self.pool.acquire().await?;

        sqlx::query!(
            r#"
            INSERT INTO posts
                (title, body)
            VALUES (?1, ?2)
            "#,
            title,
            body,
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    pub async fn search_posts(
        &self,
        query: &str,
    ) -> anyhow::Result<Vec<(Option<String>, Option<String>)>> {
        let mut conn = self.pool.acquire().await?;

        let rows = sqlx::query!(
            r#"
            SELECT
                highlight(posts,0, '<b>', '</b>') title, 
                highlight(posts,1, '<b>', '</b>') body
            FROM posts 
            WHERE posts MATCH ?1
            ORDER BY rank
            LIMIT 10
            "#,
            query
        )
        .fetch_all(&mut *conn)
        .await?
        .into_iter()
        .map(|r| (r.title, r.body))
        .collect();

        Ok(rows)
    }
}
