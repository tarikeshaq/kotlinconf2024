use std::{
    collections::HashSet,
    time::{SystemTime, UNIX_EPOCH},
};

pub use models::Question;
use network::Client;
use store::Store;

pub mod error;
mod models;
pub mod network;
pub mod store;

use error::Result;
pub struct QAManager<T, S, TB, ID> {
    network_client: T,
    store: S,
    time_provider: TB,
    id_provider: ID,
}

impl<T, S, TB, ID> QAManager<T, S, TB, ID> {
    pub fn new_with_tp(store: S, client: T, time_provider: TB, id_provider: ID) -> Self {
        Self {
            store,
            network_client: client,
            time_provider,
            id_provider,
        }
    }
}

impl<T, S, ID> QAManager<T, S, SystemTimeProvider, ID> {
    pub fn new(store: S, client: T, id_provider: ID) -> Self {
        Self {
            store,
            network_client: client,
            time_provider: SystemTimeProvider {},
            id_provider,
        }
    }
}

pub trait TimeProvider {
    fn now(&self) -> u128;
}

pub trait IDProvider {
    fn new_id(&self) -> u64;
}

pub struct SystemTimeProvider {}

impl TimeProvider for SystemTimeProvider {
    fn now(&self) -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }
}

impl<T, S, TB, ID> QAManager<T, S, TB, ID>
where
    S: Store,
    T: Client,
    TB: TimeProvider,
    ID: IDProvider,
{
    pub async fn get_local(&self) -> Result<Vec<String>> {
        let mut res = self.store.get().await?;
        res.sort_by(|a, b| a.time_created.cmp(&b.time_created));
        Ok(res.into_iter().map(|c| c.content).collect())
    }
    pub async fn write(&self, comment: &str) -> Result<()> {
        let time_created = self.time_provider.now();
        let id = self.id_provider.new_id();
        self.store
            .store(vec![Question {
                content: comment.to_owned(),
                id,
                time_created,
            }])
            .await
    }
    pub async fn sync(&self) -> Result<()> {
        let remote = self.network_client.get_remote().await?;
        let local = self.store.get().await?;
        let remote_ids = remote.iter().map(|c| c.id).collect::<HashSet<_>>();
        let local_ids = local.iter().map(|c| c.id).collect::<HashSet<_>>();
        let to_apply = remote_ids
            .difference(&local_ids)
            .into_iter()
            .map(|item| remote.iter().find(|c| c.id == *item).unwrap().clone())
            .collect::<Vec<_>>();
        let to_send = local_ids
            .difference(&remote_ids)
            .into_iter()
            .map(|item| local.iter().find(|c| c.id == *item).unwrap().clone())
            .collect::<Vec<_>>();
        self.store.store(to_apply).await?;
        self.network_client.write(to_send).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use self::store::InMemoryStore;

    use super::*;

    pub struct TestTimeProvider {
        vals: Vec<u128>,
        idx: RefCell<usize>,
    }

    impl TimeProvider for TestTimeProvider {
        fn now(&self) -> u128 {
            let mut idx = self.idx.borrow_mut();
            let val = self.vals[*idx];
            *idx += 1;
            val
        }
    }

    pub struct TestClient {}

    impl Client for TestClient {
        fn get_remote(&self) -> impl std::future::Future<Output = Result<Vec<Question>>> + Send {
            async { Ok(vec![]) }
        }

        fn write(
            &self,
            _values: Vec<Question>,
        ) -> impl std::future::Future<Output = Result<()>> + Send {
            async { Ok(()) }
        }
    }

    pub struct TestIDProvider {}

    impl IDProvider for TestIDProvider {
        fn new_id(&self) -> u64 {
            0
        }
    }

    #[tokio::test]
    async fn test_foo() {
        let conference_manager = QAManager::new_with_tp(
            InMemoryStore::default(),
            TestClient {},
            TestTimeProvider {
                vals: vec![100, 200, 300],
                idx: RefCell::new(0),
            },
            TestIDProvider {},
        );

        assert_eq!(
            Vec::<String>::new(),
            conference_manager.get_local().await.unwrap()
        );

        conference_manager.write("Hello world!").await.unwrap();
        assert_eq!(
            vec!["Hello world!"],
            conference_manager.get_local().await.unwrap()
        );
        conference_manager.write("Hello world! 2").await.unwrap();
        conference_manager.write("Hello world! 3").await.unwrap();
    }
}
