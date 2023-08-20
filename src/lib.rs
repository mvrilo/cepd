pub mod address;
pub mod error;
pub mod server;
pub mod sled;
pub mod viacep;

use crate::{address::Address, error::Error};
use async_trait::async_trait;

pub type Result<T> = std::result::Result<T, Error>;

#[async_trait]
pub trait Client: Copy + Clone + Sync + Send {
    async fn search(self, key: Vec<u8>) -> Result<Address>;
}

pub trait Storage: Clone + Sync + Send {
    fn get(&self, key: &Vec<u8>) -> Result<Address>;
    fn set(&self, key: &Vec<u8>, value: &Address) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct Cepd<C: Client = viacep::ViaCep, S: Storage = sled::Sled> {
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
