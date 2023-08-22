use cepd::{client::Viacep, storage::Sled, Cepd};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "cepd")]
#[command(bin_name = "cepd")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long)]
    dbpath: Option<String>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Query {
        postalcode: String,
    },
    Server,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let client = Viacep::default();
    let dbpath = cli.dbpath.unwrap_or_else(|| String::from("cepd_cache"));
    let storage = Sled::new(&dbpath);
    let core = Cepd::new(client, storage);
    match cli.command {
        Commands::Query { postalcode } => {
            let res = core.search(&postalcode).await.unwrap();
            println!("result: {}", res);
        }
        Commands::Server => {
            tracing_subscriber::fmt()
                .with_target(false)
                .with_max_level(tracing::Level::INFO)
                .json()
                .init();

            tracing::info!("server started at http://127.0.0.1:3000");
            cepd::server::start("127.0.0.1:3000".parse().unwrap(), core)
                .await
                .unwrap();
        }
    }
}
