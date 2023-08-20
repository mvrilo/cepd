use cepd::sled::Sled;
use cepd::viacep::ViaCep;
use cepd::Cepd;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "cepd")]
#[command(bin_name = "cepd")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Queries a zipcode
    #[command(arg_required_else_help = true)]
    Query {
        /// The zipcode to query
        zipcode: String,
    },
    /// HTTP Server
    Server,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let client = ViaCep::default();
    let storage = Sled::new("./cepd.db".into());
    let core = Cepd::new(client, storage);
    match cli.command {
        Commands::Query { zipcode } => {
            let input = zipcode.as_bytes().to_vec();
            let res = core.search(&input).await.unwrap();
            println!("result: {}", res);
        }
        Commands::Server => {
            tracing_subscriber::fmt()
                .with_target(false)
                .with_max_level(tracing::Level::DEBUG)
                .json()
                .init();

            tracing::info!("server started at http://127.0.0.1:3000");
            cepd::server::start("127.0.0.1:3000".parse().unwrap(), core)
                .await
                .unwrap();
        }
    }
}
