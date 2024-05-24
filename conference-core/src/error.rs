#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Erroring storing")]
    StoreError,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
