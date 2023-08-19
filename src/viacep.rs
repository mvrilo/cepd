use crate::{Address, Client, Error};
use async_trait::async_trait;

#[derive(Debug, Clone, Copy, Default)]
pub struct ViaCep;

#[async_trait]
impl Client for ViaCep {
    async fn search(self, postalcode: Vec<u8>) -> Result<Address, Error> {
        let res = reqwest::get(format!(
            "https://viacep.com.br/ws/{}/json/",
            String::from_utf8_lossy(&postalcode),
        ))
        .await?;
        Ok(res.json().await?)
    }
}
