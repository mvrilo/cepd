use cepd::{client::Viacep, storage::Sled, Cepd, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let tmp = std::env::temp_dir().display().to_string();
    let cepd: Cepd<Viacep, Sled> = Cepd::with_storage(Sled::new(&tmp));
    let address = cepd.search("01311200").await?;
    println!("city: {}", address.city);
    Ok(())
}
