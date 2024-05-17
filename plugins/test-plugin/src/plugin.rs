#[allow(unused_imports)]
pub use atlascli::plugin::host;
use clap::CommandFactory;
use exports::atlascli::plugin::info::Guest;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::{args::Cli, plugin_main};

wit_bindgen::generate!({
    path: "../../wit/plugin.wit",
    world: "plugin"
});

const PLUGIN_NAME: &str = "test-plugin";
struct Plugin;

export!(Plugin);

impl Guest for Plugin {
    fn name() -> String {
        PLUGIN_NAME.to_string()
    }

    fn run() -> Result<(), String> {
        setup_logging();
        plugin_main().map_err(|e| e.to_string())?;
        Ok(())
    }

    fn sub_commands() -> Vec<String> {
        Cli::command()
            .get_subcommands()
            .map(|c| c.get_name().to_string())
            .collect()
    }
}

fn setup_logging() {
    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
}
