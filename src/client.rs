use crate::{Address, Result};
use async_trait::async_trait;

const HOST_VIACEP: &'static str = "https://viacep.com.br/ws";

#[async_trait]
pub trait Client: Copy + Clone + Sync + Send {
    async fn search(self, postalcode: &str) -> Result<Address>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Viacep;

#[async_trait]
impl Client for Viacep {
    async fn search(self, postalcode: &str) -> Result<Address> {
        let url = format!("{}/{}/json/", HOST_VIACEP, &postalcode);
        let res = reqwest::get(url).await?;
        let addr = res.json().await?;
        Ok(addr)
    }
}
