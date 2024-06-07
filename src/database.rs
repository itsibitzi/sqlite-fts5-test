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

    pub async fn insert_enron_email(&self, title: &str, body: &str) -> anyhow::Result<()> {
        let mut conn = self.pool.acquire().await?;

        sqlx::query!(
            r#"
            INSERT INTO enron_emails
                (file, message)
            VALUES (?1, ?2)
            "#,
            title,
            body,
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    pub async fn search_enron_emails(
        &self,
        query: &str,
    ) -> anyhow::Result<Vec<(Option<String>, Option<String>)>> {
        let mut conn = self.pool.acquire().await?;

        let rows = sqlx::query!(
            r#"
            SELECT
                highlight(enron_emails,0, '<b>', '</b>') file, 
                highlight(enron_emails,1, '<b>', '</b>') message
            FROM enron_emails 
            WHERE enron_emails MATCH ?1
            ORDER BY rank
            LIMIT 10
            "#,
            query
        )
        .fetch_all(&mut *conn)
        .await?
        .into_iter()
        .map(|r| (r.file, r.message))
        .collect();

        Ok(rows)
    }
}
