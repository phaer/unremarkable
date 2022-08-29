pub mod notebooks;
pub mod api;

use anyhow::Result;
use clap::Parser;



#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
   #[clap(short, long, value_parser, default_value = "localhost")]
   host: String,

   #[clap(short, long, value_parser, default_value_t = 8080)]
   port: u16,
}

#[actix_web::main]
async fn main() -> Result<()> {
   let cli = Cli::parse();

    api::start(cli.host, cli.port).await?;

    Ok(())
}
