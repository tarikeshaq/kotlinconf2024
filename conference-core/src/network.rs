use crate::{error::Result, models::Question};

pub trait Client {
    fn get_remote(&self) -> impl std::future::Future<Output = Result<Vec<Question>>>;
    fn write(&self, values: Vec<Question>) -> impl std::future::Future<Output = Result<()>>;
}
