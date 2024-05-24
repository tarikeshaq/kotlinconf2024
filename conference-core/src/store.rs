use std::sync::Mutex;

use crate::models::Question;

use super::error::Result;
pub trait Store: Send + Sync {
    fn store(&self, comment: Vec<Question>)
        -> impl std::future::Future<Output = Result<()>> + Send;
    fn get(&self) -> impl std::future::Future<Output = Result<Vec<Question>>> + Send;
}

#[derive(Default)]
pub struct InMemoryStore {
    db: Mutex<Vec<Question>>,
}

impl Store for InMemoryStore {
    async fn store(&self, mut comments: Vec<Question>) -> Result<()> {
        Ok(self.db.lock().unwrap().append(&mut comments))
    }
    async fn get(&self) -> Result<Vec<Question>> {
        Ok(self.db.lock().unwrap().clone())
    }
}
