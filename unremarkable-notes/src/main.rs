pub mod notebooks;

use std::io::Read;
use std::net::TcpStream;
use anyhow::{Context, Result};
use clap::Parser;
use ssh2::{Session, Channel};



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

fn main() -> Result<()> {
   let cli = Cli::parse();

    let session = connect(
        cli.ip,
        cli.port,
        cli.username,
        cli.password
    ).context("Failed to connect to your remarkable")?;

    let sftp = session.sftp().context("Failed to connect via sftp")?;
    let notebooks = notebooks::list_notebooks(sftp).context("Failed to list notebooks")?;
    for notebook in notebooks {
        println!("{:?}", notebook)
    }

    //let mut channel = session.channel_session()
    //    .context("Could not open SSH channel")?;

    //let output = exec(&mut channel, "uname -a")?;
    //println!("{}", output);

    Ok(())
}
