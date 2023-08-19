use cepd::client::viacep::ViaCep;
use cepd::storage::sled::Sled;
use cepd::Cepd;

// #[tokio::main]
fn main() {
    let client = ViaCep::default();
    let storage = Sled::new("./cepd.db".into());
    let core = Cepd::new(&client, &storage);
    let input = "04207030".as_bytes().to_vec();
    let res = core.search(&input).unwrap();
    println!("result: {}", res);
}
