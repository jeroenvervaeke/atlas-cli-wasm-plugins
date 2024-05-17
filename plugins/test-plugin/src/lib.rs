use anyhow::{Context, Result};
use clap::Parser;
use tracing::{info, trace};

use crate::args::Cli;

mod args;
mod plugin;

pub fn plugin_main() -> Result<()> {
    let cli = Cli::parse();

    match cli.sub_command {
        args::Command::TestHost => call_host(),
        args::Command::PrintEnv => print_env(),
        args::Command::PrintStdIn => print_stdin(),
    }
}

#[tracing::instrument]
fn call_host() -> Result<()> {
    trace!("about to call host");

    let bearer_token = plugin::host::bearer_token();
    info!(?bearer_token, "received bearer token");

    Ok(())
}

#[tracing::instrument]
fn print_env() -> Result<()> {
    for (key, value) in std::env::vars() {
        info!("{key}={value}")
    }

    Ok(())
}

#[tracing::instrument]
fn print_stdin() -> Result<()> {
    let mut stdout = std::io::stdout().lock();
    let mut stdin = std::io::stdin().lock();

    std::io::copy(&mut stdin, &mut stdout).context("copy stdin to stdout")?;

    Ok(())
}
