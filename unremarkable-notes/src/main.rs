use std::io::Read;
use std::path::{Path, PathBuf};
use std::net::TcpStream;
use clap::Parser;
use anyhow::{Context, Result};
use ssh2::{Session, Channel, Sftp};
use serde::{Deserialize, Serialize};

const REMARKABLE_NOTEBOOK_STORAGE_PATH: &str = "/home/root/.local/share/remarkable/xochitl/";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
   #[clap(short, long, value_parser)]
   ip: String,

   #[clap(short, long, value_parser, default_value_t = 22)]
   port: u16,

   #[clap(short, long, value_parser)]
   username: Option<String>,

   #[clap(long, value_parser)]
   password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NotebookMeta {
    deleted: bool,
    last_modified: String,
    #[serde(default)]
    last_opened: Option<String>,
    #[serde(default)]
    last_opened_page: Option<u16>,
    metadatamodified: bool,
    modified: bool,
    parent: String,
    pinned: bool,
    synced: bool,
    #[serde(rename = "type")]
    type_: String,
    version: u8,
    visible_name: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Notebook {
    name: String,
    path: PathBuf,
    metadata: NotebookMeta
}



fn connect(
    ip: String,
    port: u16,
    username: Option<String>,
    password: Option<String>
) -> Result<Session> {
    let target = format!("{}:{}", ip, port);
    let tcp = TcpStream::connect(target)
        .with_context(|| format!("Failed to connect to {}:{}", ip, port))?;
    let mut session = Session::new().context("Could not create SSH session")?;
    session.set_tcp_stream(tcp);
    session.handshake().context("Could not finish SSH handshake")?;

    let username = username.unwrap_or(String::from("root"));
    if let Some(password) = password {
        session.userauth_password(username.as_str(), password.as_ref())
            .context("Could not authenticate with password")?;
    } else {
        session.userauth_agent(username.as_str())
            .context("Could not authenticate via SSH agent")?;
    }
    assert!(session.authenticated());
    Ok(session)
}

fn exec(
    channel: &mut Channel,
    command: &str
) -> Result<String> {
    let mut s = String::new();
    channel.exec(command).context("Could not execute command")?;
    channel.read_to_string(&mut s).context("Could not read from channel")?;
    Ok(s)
}

fn list_notebooks(
    sftp: Sftp
) -> Result<Vec<Notebook>> {
    let path = Path::new(REMARKABLE_NOTEBOOK_STORAGE_PATH);
    let mut result = Vec::new();
    let files = sftp.readdir(path).context("Could not list files in storage directory")?;
    for (path_buffer, _file_stat) in files {
        if path_buffer.extension().map_or (false, |v| v == "metadata") {
            let metadata_file = sftp
                .open(&path_buffer)
                .with_context(|| format!("Could not read metadata {:?}", path_buffer))?;
            let metadata: NotebookMeta = serde_json::from_reader(metadata_file)
                .with_context(|| format!("Could not parse metadata at {:?}", path_buffer))?;
            let notebook = Notebook {
                name: metadata.visible_name.clone(),
                path: path_buffer.with_extension(""),
                metadata
            };
            result.push(notebook);
       }
    }
    Ok(result)
}

fn main() -> Result<()> {
   let cli = Cli::parse();

    let session = connect(
        cli.ip,
        cli.port,
        cli.username,
        cli.password
    ).context("Failed to connect to your remarkable")?;

    let sftp = session.sftp().context("Failed to connect via sftp")?;
    let notebooks = list_notebooks(sftp).context("Failed to list notebooks")?;
    for notebook in notebooks {
        println!("{:?}", notebook)
    }

    //let mut channel = session.channel_session()
    //    .context("Could not open SSH channel")?;

    //let output = exec(&mut channel, "uname -a")?;
    //println!("{}", output);

    Ok(())
}
