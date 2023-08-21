use crate::{Address, Error, Result};
use std::sync::Arc;

pub trait Storage: Clone + Sync + Send {
    fn get(&self, key: &Vec<u8>) -> Result<Address>;
    fn set(&self, key: &Vec<u8>, value: &Address) -> Result<()>;
}

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
    fn get(&self, key: &Vec<u8>) -> Result<Address> {
        match self.db.get(key)? {
            Some(val) => Ok(bincode::deserialize(val.as_ref().into())?),
            None => Err(Error::CacheMiss),
        }
    }

    fn set(&self, key: &Vec<u8>, addr: &Address) -> Result<()> {
        let val = bincode::serialize(addr)?;
        self.db.insert(key, val)?;
        Ok(())
    }
}
