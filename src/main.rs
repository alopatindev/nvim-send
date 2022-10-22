use anyhow::{format_err, Context, Result};
use clap::{
    arg, builder::Command, crate_authors, crate_description, crate_name, crate_version, ArgMatches,
};
use futures::io::AsyncWrite;
use nvim_rs::Neovim;
use nvim_rs::{
    create::tokio::{new_path, new_tcp},
    rpc::handler::Dummy,
};

async fn process_args<T: AsyncWrite + Send + Unpin + 'static>(
    neovim: &Neovim<T>,
    matches: ArgMatches,
) -> Result<()> {
    let args = matches
        .ids()
        .map(|i| i.as_str())
        .filter(|i| i != &"servername");
    for i in args {
        if let Some(value) = matches.get_one::<String>(i) {
            if i == "command" {
                neovim.command(value).await?;
            } else if i == "remote-send" {
                neovim.input(value).await?;
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!("\n"))
        .arg(arg!(--command <command> "Execute command").required(false))
        .arg(arg!(--"remote-send" <keys> "Send key presses").required(false))
        .arg(arg!(--servername <address> "Set the address to be used"))
        .get_matches();

    let server_address = matches
        .get_one::<String>("servername")
        .context("servername not found")?;

    let handler = Dummy::new();
    if let Ok((neovim, _job_handler)) = new_tcp(server_address, handler).await {
        process_args(&neovim, matches).await?;
        return Ok(());
    } else {
        let handler = Dummy::new();
        if let Ok((neovim, _job_handler)) = new_path(server_address, handler).await {
            process_args(&neovim, matches).await?;
            return Ok(());
        }
    }

    Err(format_err!("cannot connect"))
}
