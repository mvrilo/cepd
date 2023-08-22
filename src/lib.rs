pub mod address;
pub mod client;
pub mod error;
pub mod server;
pub mod storage;

use crate::{address::Address, client::Client, error::Error, storage::Storage};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct Cepd<C: Client, S: Storage> {
    pub storage: S,
    pub client: C,
}

impl<C, S> Cepd<C, S>
where
    C: Client,
    S: Storage,
{
    pub fn new(client: C, storage: S) -> Self {
        Self { storage, client }
    }
}

impl<C: Client, S: Storage> Cepd<C, S> {
    pub async fn search(&self, cep: &Vec<u8>) -> Result<Address> {
        let cli = self.client.clone();
        let storage = self.storage.clone();
        match storage.get(cep) {
            Ok(addr) => Ok(addr),
            Err(_) => {
                let addr = cli.search(cep.clone()).await?;
                storage.set(cep, &addr)?;
                Ok(addr)
            }
        }
    }
}
