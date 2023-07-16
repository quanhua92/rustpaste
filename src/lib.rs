use async_graphql::{Context, EmptySubscription, Object, Schema};
use nanoid::nanoid;

pub type ServiceSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
pub struct QueryRoot;
pub struct MutationRoot;

mod storage;
pub use storage::Storage;

#[derive(Clone)]
pub struct Paste {
    id: String,
    title: String,
    content: String,
    password: Option<String>,
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

    pub async fn all_pastes(&self, ctx: &Context<'_>) -> Vec<Paste> {
        let storage = ctx.data_unchecked::<Storage>().lock().await;
        storage.get_all()
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
    ) -> Paste {
        let mut storage = ctx.data_unchecked::<Storage>().lock().await;
        let id = nanoid!();
        let paste = Paste {
            id: id.clone(),
            title,
            content,
            password,
        };

        let result = storage.insert(&id, &paste);
        match result {
            None => paste,
            Some(p) => p,
        }
    }

    async fn delete_paste(&self, ctx: &Context<'_>, id: String, password: Option<String>) -> bool {
        let mut storage = ctx.data_unchecked::<Storage>().lock().await;
        let paste = storage.get(&id);

        match paste {
            None => true,
            Some(paste) => match paste.password {
                None => {
                    storage.remove(&id);
                    true
                }
                Some(stored_pass) => match password {
                    None => false,
                    Some(input_pass) => {
                        if stored_pass == input_pass {
                            storage.remove(&id);
                            return true;
                        }
                        false
                    }
                },
            },
        }
    }
}
