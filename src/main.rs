use anyhow::Context;
use anyhow::Result;
use clap::App;
use nvim_rs::{
    create::tokio::{new_path, new_tcp},
    rpc::handler::Dummy,
};
use tokio::net::lookup_host;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = App::new("nvim-send")
        .args_from_usage(
            "--remote-send=[keys]     'Send key presses'
             --servername=[address]   'Set the address to be used'",
        )
        .get_matches();

    let server_address = matches
        .value_of("servername")
        .context("servername not found")?;

    let keys = matches.value_of("remote-send").context("keys not found")?;

    let bytes_written = if lookup_host(server_address).await.is_ok() {
        let handler = Dummy::new();
        let (neovim, _job_handler) = new_tcp(server_address, handler).await?;
        neovim.input(keys).await?
    } else {
        let handler = Dummy::new();
        let (neovim, _job_handler) = new_path(server_address, handler).await?;
        neovim.input(keys).await?
    };
    debug_assert_eq!(bytes_written as usize, keys.as_bytes().len());

    Ok(())
}
