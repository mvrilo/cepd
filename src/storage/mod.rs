pub mod sled;
use crate::{Address, Error};

pub trait Storage {
    fn get(&self, key: Vec<u8>) -> Result<Address, Error>;
    fn set(&self, key: &Vec<u8>, value: &Address) -> Result<(), Error>;
}
