use anyhow::Context;
use anyhow::Result;
use clap::{crate_authors, crate_description, crate_name, crate_version, App};
use nvim_rs::{
    create::tokio::{new_path, new_tcp},
    rpc::handler::Dummy,
};

#[tokio::main]
async fn main() -> Result<()> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!("\n"))
        .args_from_usage(
            "--command=[command]      'Execute command'
             --remote-send=[keys]     'Send key presses'
             --servername=[address]   'Set the address to be used'",
        )
        .get_matches();

    let server_address = matches
        .value_of("servername")
        .context("servername not found")?;

    let handler = Dummy::new();
    if let Ok((neovim, _job_handler)) = new_tcp(server_address, handler).await {
        if let Some(command) = matches.value_of("command") {
            neovim.command(command).await?;
        }
        if let Some(keys) = matches.value_of("remote-send") {
            neovim.input(keys).await?;
        }
    } else {
        let handler = Dummy::new();
        if let Ok((neovim, _job_handler)) = new_path(server_address, handler).await {
            if let Some(command) = matches.value_of("command") {
                neovim.command(command).await?;
            }
            if let Some(keys) = matches.value_of("remote-send") {
                neovim.input(keys).await?;
            }
        }
    }

    Ok(())
}
