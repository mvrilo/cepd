pub mod address;
pub mod client;
pub mod error;
pub mod storage;

use crate::address::Address;
use crate::client::Client;
use crate::error::Error;
use crate::storage::Storage;

#[derive(Default, Debug)]
pub struct Cepd<'a, C: Client + Copy, S: Storage> {
    pub storage: Option<&'a S>,
    pub client: Option<&'a C>,
}

impl<'a, C: Client + Copy, S: Storage> Cepd<'a, C, S> {
    pub fn search(&self, cep: &'a Vec<u8>) -> Result<Address, Error> {
        let storage = self.storage.clone().unwrap();
        let cli = self.client.clone().unwrap();

        storage
            .get(cep.clone())
            .or_else(|_| cli.search(cep.clone()))
    }
}
