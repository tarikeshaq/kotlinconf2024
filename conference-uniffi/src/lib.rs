uniffi::setup_scaffolding!();
use std::sync::Arc;

use conference_core::{
    error::Result, network::Client, store::InMemoryStore, IDProvider, QAManager as InnerQAManager,
    Question, SystemTimeProvider,
};

#[derive(uniffi::Record)]
pub struct QuestionRecord {
    content: String,
    id: u64,
    time_created: u64,
}

impl Into<Question> for QuestionRecord {
    fn into(self) -> Question {
        Question {
            content: self.content,
            id: self.id,
            time_created: self.time_created as u128,
        }
    }
}

impl From<Question> for QuestionRecord {
    fn from(value: Question) -> Self {
        QuestionRecord {
            content: value.content,
            id: value.id,
            time_created: value.time_created as u64,
        }
    }
}

struct RandIdProvider {}

impl IDProvider for RandIdProvider {
    fn new_id(&self) -> u64 {
        rand::random::<u32>() as u64
    }
}

#[derive(uniffi::Object)]
pub struct QAManager {
    inner: InnerQAManager<UniFFIHttpClient, InMemoryStore, SystemTimeProvider, RandIdProvider>,
}

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait HttpClient: Send + Sync + std::fmt::Debug {
    async fn get(&self) -> Vec<QuestionRecord>;
    async fn write(&self, values: Vec<QuestionRecord>);
}

#[derive(Debug)]
pub struct UniFFIHttpClient {
    inner: Arc<dyn HttpClient>,
}

impl Client for UniFFIHttpClient {
    fn get_remote(&self) -> impl std::future::Future<Output = Result<Vec<Question>>> + Send {
        async {
            Ok(self
                .inner
                .get()
                .await
                .into_iter()
                .map(|c| c.into())
                .collect())
        }
    }

    fn write(&self, values: Vec<Question>) -> impl std::future::Future<Output = Result<()>> + Send {
        async {
            Ok(self
                .inner
                .write(values.into_iter().map(Into::into).collect())
                .await)
        }
    }
}

#[uniffi::export]
impl QAManager {
    #[uniffi::constructor]
    pub fn new(http_client: Arc<dyn HttpClient>) -> Self {
        Self {
            inner: InnerQAManager::new(
                InMemoryStore::default(),
                UniFFIHttpClient { inner: http_client },
                RandIdProvider {},
            ),
        }
    }

    pub async fn get(&self) -> Vec<String> {
        self.inner.get_local().await.unwrap()
    }

    pub async fn write(&self, value: &str) {
        self.inner.write(value).await.unwrap()
    }

    pub async fn sync(&self) {
        self.inner.sync().await.unwrap()
    }
}
