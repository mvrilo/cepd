pub mod address;
pub mod client;
pub mod error;
pub mod storage;

use crate::address::Address;
use crate::client::Client;
use crate::error::Error;
use crate::storage::Storage;

#[derive(Debug)]
pub struct Cepd<'a, C: Client + Copy, S: Storage> {
    pub storage: &'a S,
    pub client: &'a C,
}

impl<'a, C: Client + Copy, S: Storage> Cepd<'a, C, S> {
    pub fn new(client: &'a C, storage: &'a S) -> Self {
        Self { storage, client }
    }
}

impl<'a, C: Client + Copy, S: Storage> Cepd<'a, C, S> {
    pub fn search(&self, cep: &'a Vec<u8>) -> Result<Address, Error> {
        let storage = self.storage.clone();
        let cli = self.client.clone();
        storage.get(cep).or_else(|_| cli.search(cep.clone()))
    }
}
