use anyhow::{Context, Result};
use plugin::Manager;

mod plugin;

#[tokio::main]
async fn main() -> Result<()> {
    let mut plugin_manager = Manager::load_directory("plugins")
        .await
        .context("load directory")?;

    let subcommands = plugin_manager
        .subcommands()
        .await
        .context("get subcommands")?;

    match std::env::args().nth(1).as_deref() {
        Some("-h") | Some("--help") | None => print_usage(subcommands),
        Some(subcommand) => {
            match plugin_manager
                .run_sub_command(subcommand)
                .await
                .context("run subcommand")?
            {
                plugin::RunSubCommandOutcome::Ok => {}
                plugin::RunSubCommandOutcome::NotFound => print_usage(subcommands),
            }
        }
    }

    Ok(())
}

fn print_usage(subcommands: Vec<String>) {
    println!("Usage: atlas-cli-ng [COMMAND]");
    println!();
    println!("Commands:");
    for subcommand in subcommands {
        println!("  {subcommand}");
    }
    println!();

    println!("Options:");
    println!("  -h, --help    Print help");
    println!("  -V, --version Print version");
}