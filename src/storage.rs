use crate::{Address, Error, Result};
use anyhow::Context;
use std::sync::Arc;

pub trait Storage: Clone + Sync + Send {
    fn get(&self, key: &str) -> Result<Address>;
    fn set(&self, key: &str, value: &Address) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct Sled {
    db: Arc<sled::Db>,
}

impl Sled {
    pub fn new(path: &str) -> Self {
        Sled {
            db: Arc::new(
                sled::open(path)
                    .context("Failed to open sled database")
                    .unwrap(),
            ),
        }
    }
}

impl Storage for Sled {
    fn get(&self, key: &str) -> Result<Address> {
        match self.db.get(key)? {
            Some(val) => Ok(bincode::deserialize(val.as_ref().into())?),
            None => Err(Error::CacheMiss),
        }
    }

    fn set(&self, key: &str, addr: &Address) -> Result<()> {
        let val = bincode::serialize(addr)?;
        self.db.insert(key, val)?;
        Ok(())
    }
}
