use crate::{Address, Result};
use async_trait::async_trait;

const HOST_VIACEP: &'static str = "https://viacep.com.br/ws";

#[async_trait]
pub trait Client: Copy + Clone + Sync + Send {
    async fn search(self, key: Vec<u8>) -> Result<Address>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ViaCep;

#[async_trait]
impl Client for ViaCep {
    async fn search(self, postalcode: Vec<u8>) -> Result<Address> {
        let url = format!(
            "{}/{}/json/",
            HOST_VIACEP,
            String::from_utf8_lossy(&postalcode),
        );
        let res = reqwest::get(url).await?;
        Ok(res.json().await?)
    }
}
