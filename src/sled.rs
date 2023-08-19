use crate::{Address, Error, Storage};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Sled {
    db: Arc<sled::Db>,
}

impl Sled {
    pub fn new(path: &str) -> Self {
        Sled {
            db: Arc::new(sled::open(path).unwrap()),
        }
    }
}

impl Storage for Sled {
    fn get(&self, key: &Vec<u8>) -> Result<Address, Error> {
        match self.db.get(key)? {
            Some(val) => Ok(bincode::deserialize(val.as_ref().into())?),
            None => Err(Error::CacheMiss),
        }
    }

    fn set(&self, key: &Vec<u8>, addr: &Address) -> Result<(), Error> {
        let val = bincode::serialize(addr)?;
        self.db.insert(key, val)?;
        Ok(())
    }
}
