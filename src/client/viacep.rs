use crate::{Address, Client, Error};

#[derive(Debug, Clone, Copy, Default)]
pub struct ViaCep;

impl Client for ViaCep {
    fn search(self, postalcode: Vec<u8>) -> Result<Address, Error> {
        let res = reqwest::blocking::get(format!(
            "https://viacep.com.br/ws/{}/json/",
            String::from_utf8_lossy(&postalcode),
        ))?;
        Ok(res.json()?)
    }
}
