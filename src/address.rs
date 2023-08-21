use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Address {
    // #[serde(skip_serializing)]
    // pub ibge: String,

    // #[serde(skip_serializing)]
    // pub gia: String,
    #[serde(rename(deserialize = "cep"))]
    pub postalcode: String,

    #[serde(rename(deserialize = "logradouro"))]
    pub address: String,

    #[serde(rename(deserialize = "complemento"))]
    pub complement: String,

    #[serde(rename(deserialize = "bairro"))]
    pub neighborhood: String,

    #[serde(rename(deserialize = "localidade"))]
    pub city: String,

    #[serde(rename(deserialize = "uf"))]
    pub state: String,
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}, {} - {} - {} - {}",
            self.address, self.complement, self.neighborhood, self.city, self.state
        )
    }
}
