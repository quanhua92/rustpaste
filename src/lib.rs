use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};

pub type ServiceSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    pub async fn hello(&self) -> &'static str {
        "Hello RustPaste"
    }
}
