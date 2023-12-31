use sqlx::{Pool, Postgres};

use crate::{Paste, PasteError};

pub struct PasteStorage {
    pool: Pool<Postgres>,
}

impl PasteStorage {
    pub fn new(pool: Pool<Postgres>) -> Self {
        PasteStorage { pool }
    }
    pub async fn insert(
        &mut self,
        id: String,
        title: String,
        content: String,
        password: Option<String>,
    ) -> Result<Paste, PasteError> {
        let paste = Paste {
            id,
            title,
            content,
            password,
        };

        sqlx::query!(
            "INSERT INTO paste VALUES ($1, $2, $3, $4)",
            paste.id,
            paste.title,
            paste.content,
            paste.password
        )
        .execute(&self.pool)
        .await?;
        Ok(paste)
    }

    pub async fn update(
        &mut self,
        id: String,
        title: String,
        content: String,
        password: Option<String>,
    ) -> Result<Paste, PasteError> {
        let paste = Paste {
            id,
            title,
            content,
            password,
        };

        sqlx::query!(
            "UPDATE paste set title=$2, content=$3, password=$4 where id=$1",
            paste.id,
            paste.title,
            paste.content,
            paste.password
        )
        .execute(&self.pool)
        .await?;
        Ok(paste)
    }

    pub async fn remove(&mut self, id: &str) -> Result<(), PasteError> {
        sqlx::query!("DELETE FROM paste WHERE id=$1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get(&self, id: &str) -> Result<Option<Paste>, PasteError> {
        let result = sqlx::query!(
            "SELECT id, title, content, password FROM paste WHERE id=$1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some(row) => Ok(Some(Paste {
                id: row.id,
                title: row.title,
                content: row.content,
                password: row.password,
            })),
            None => Ok(None),
        }
    }

    pub async fn get_all(&self) -> Result<Vec<Paste>, PasteError> {
        let results = sqlx::query!("SELECT id, title, content, password FROM paste")
            .fetch_all(&self.pool)
            .await?;
        let mut pastes = vec![];
        for row in results {
            let p = Paste {
                id: row.id,
                title: row.title,
                content: row.content,
                password: row.password,
            };
            pastes.push(p);
        }
        Ok(pastes)
    }
}
