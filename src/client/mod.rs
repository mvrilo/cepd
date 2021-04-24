pub mod viacep;

use crate::{Address, Error};

pub trait Client {
    fn search(self, key: Vec<u8>) -> Result<Address, Error>;
}
