pub mod notebooks;
pub mod api;
pub mod pdf;

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
   #[clap(short, long, value_parser, default_value = "localhost:8080")]
   addr: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    api::start(cli.addr).await?;

    Ok(())
}
