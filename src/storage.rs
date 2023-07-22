use std::sync::Arc;

use async_graphql::futures_util::lock::Mutex;
use sqlx::{Pool, Postgres};

use crate::Paste;

pub struct PasteStorage {
    pool: Pool<Postgres>,
}

pub type Storage = Arc<Mutex<PasteStorage>>;

impl PasteStorage {
    pub fn new(pool: Pool<Postgres>) -> Self {
        PasteStorage { pool }
    }
    pub async fn insert(&mut self, key: &str, value: &Paste) -> Option<Paste> {
        let results = sqlx::query!(
            "INSERT INTO paste VALUES ($1, $2, $3, $4)",
            value.id,
            value.title,
            value.content,
            value.password
        )
        .execute(&self.pool)
        .await;
        match results {
            Ok(_) => Some(value.clone()),
            Err(_) => None,
        }
    }

    pub async fn remove(&mut self, key: &str) -> Option<Paste> {
        let results = sqlx::query!("DELETE FROM paste WHERE id=$1", key)
            .execute(&self.pool)
            .await;
        match results {
            Ok(_) => Some(Paste {
                id: key.to_string(),
                title: "deleted".to_string(),
                content: "deleted".to_string(),
                password: None,
            }),
            Err(_) => None,
        }
    }

    pub async fn get(&self, key: &str) -> Option<Paste> {
        let results = sqlx::query!(
            "SELECT id, title, content, password FROM paste WHERE id=$1",
            key
        )
        .fetch_optional(&self.pool)
        .await;
        match results {
            Ok(Some(row)) => Some(Paste {
                id: row.id,
                title: row.title,
                content: row.content,
                password: row.password,
            }),
            Ok(None) => None,
            Err(_) => None,
        }
    }

    pub async fn get_all(&self) -> Vec<Paste> {
        let results = sqlx::query!("SELECT id, title, content, password FROM paste")
            .fetch_all(&self.pool)
            .await;
        match results {
            Ok(results) => {
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
                pastes
            }
            Err(_) => vec![],
        }
    }
}
