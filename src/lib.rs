use async_graphql::{Context, EmptySubscription, Object, Schema};
use nanoid::nanoid;
use thiserror::Error;

pub type ServiceSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
pub struct QueryRoot;
pub struct MutationRoot;

mod storage;
pub use storage::{PasteStorage, Storage};

#[derive(Clone)]
pub struct Paste {
    id: String,
    title: String,
    content: String,
    password: Option<String>,
}

#[derive(Debug, Error)]
pub enum PasteError {
    #[error("invalid id")]
    InvalidId,
    #[error("invalid password")]
    InvalidPassword,
    #[error("database error")]
    DatabaseError(#[from] sqlx::Error),
}

#[Object]
impl Paste {
    async fn id(&self) -> &str {
        &self.id
    }
    async fn title(&self) -> &str {
        &self.title
    }
    async fn content(&self) -> &str {
        &self.content
    }
}

#[Object]
impl QueryRoot {
    pub async fn hello(&self) -> &'static str {
        "Hello RustPaste"
    }

    pub async fn all_pastes(&self, ctx: &Context<'_>) -> Result<Vec<Paste>, PasteError> {
        let storage = ctx.data_unchecked::<Storage>().lock().await;
        storage.get_all().await
    }

    pub async fn paste(&self, ctx: &Context<'_>, id: String) -> Result<Option<Paste>, PasteError> {
        let storage = ctx.data_unchecked::<Storage>().lock().await;
        storage.get(&id).await
    }
}

#[Object]
impl MutationRoot {
    async fn create_paste(
        &self,
        ctx: &Context<'_>,
        title: String,
        content: String,
        password: Option<String>,
    ) -> Result<Paste, PasteError> {
        let mut storage = ctx.data_unchecked::<Storage>().lock().await;
        let id = nanoid!();

        storage.insert(id, title, content, password).await
    }

    async fn delete_paste(
        &self,
        ctx: &Context<'_>,
        id: String,
        password: Option<String>,
    ) -> Result<bool, PasteError> {
        let mut storage = ctx.data_unchecked::<Storage>().lock().await;
        let paste = storage.get(&id).await?;

        match paste {
            None => Err(PasteError::InvalidId),
            Some(paste) => match paste.password {
                None => {
                    storage.remove(&id).await?;
                    Ok(true)
                }
                Some(stored_pass) => match password {
                    None => Err(PasteError::InvalidPassword),
                    Some(input_pass) => {
                        if stored_pass == input_pass {
                            storage.remove(&id).await?;
                            return Ok(true);
                        }
                        Err(PasteError::InvalidPassword)
                    }
                },
            },
        }
    }
}
