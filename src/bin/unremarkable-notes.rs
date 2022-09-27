use std::path::PathBuf;
use clap::{Parser, Subcommand};
use unremarkable_notes::{config, sync, storage};
use unremarkable_notes::storage::{Store, ItemType};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Sync {
        #[clap(subcommand)]
        command: SyncCommands,
    },
    Store {
        #[clap(subcommand)]
        command: StoreCommands,
    },
}

#[derive(Subcommand)]
/// Interact with the official API or rmfakecloud
enum SyncCommands {
    /// Show information about the configured sync server
    Info {},
}

#[derive(Subcommand)]
/// Interact with locally stored files
enum StoreCommands {
    /// List all documents
    List {},
    /// Render a given document
    Render {
        #[clap(value_parser)]
        id: String,
    },
}

fn main() {
    let cli = Cli::parse();

    // TODO use https://crates.io/crates/envy instead
    let path = PathBuf::from("config.toml");
    let config = match config::Config::from_file(&path) {
        Err(e) => panic!("{}", e),
        Ok(v) => v
    };

    match &cli.command {
        Commands::Sync { command } => {
            let client = sync::Client::from(config).unwrap();
            match command {
                SyncCommands::Info {  } => {
                    client.info();
                }
            }
        }
        Commands::Store { command } => {
            let store = storage::FileSystemStore::default();
            match command {
                StoreCommands::List {  } => {
                    let items = match store.all() {
                        Err(e) => panic!("Could not list files: {}", e),
                        Ok(v) => v
                    };
                    for item in items {
                        println!("{}", item)
                    }
                }
                StoreCommands::Render { id } => {
                   let document = match store.load(id) {
                        Err(e) => panic!("Could not load document: {}", e),
                        Ok(v) => match v {
                            ItemType::Document(d) => d,
                            ItemType::Collection(_) => panic!("Can't render a collection")
                        }
                    };
                    if let Err(e) = document.to_svg(&store, &PathBuf::from("test.svg"), 1) {
                        panic!("Could not load document: {}", e);
                    }
                }
            }
        }
    }
}
