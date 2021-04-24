use cepd::client::viacep::ViaCep;
use cepd::storage::sled::Sled;
use cepd::Cepd;

// #[tokio::main]
fn main() {
    let client = ViaCep::default();
    let storage = Sled::new("./cepd.db".into());

    let svc = Cepd {
        client: Some(&client),
        storage: Some(&storage),
    };

    let input = "04207030";
    let code = input.as_bytes().to_vec();

    match svc.search(&code) {
        Ok(address) => println!("code: {} {}", input, address),
        Err(err) => panic!(err),
    }
}
