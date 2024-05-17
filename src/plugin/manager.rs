use std::collections::HashSet;
use std::path::Path;
use std::sync::Arc;

use super::component::atlascli::plugin::host::{self};
use anyhow::{Context, Result};
use tokio::fs::read_dir;
use wasmtime::component::Linker;
use wasmtime::{Config, Engine};
use wasmtime_wasi::add_to_linker_async;

use crate::plugin::plugin::{Plugin, PluginState};

pub struct Manager {
    plugins: Vec<Plugin>,
}

impl Manager {
    pub async fn load_directory(plugin_directory: impl AsRef<Path>) -> Result<Self> {
        let mut directory_entries = read_dir(plugin_directory)
            .await
            .context("read plugin directory")?;

        let mut plugin_paths = HashSet::new();
        let mut compiled_plugin_paths = HashSet::new();

        while let Some(entry) = directory_entries
            .next_entry()
            .await
            .context("loop through plugin directory")?
        {
            if entry.file_type().await.context("get file type")?.is_file() {
                let file_path = entry.path();

                match file_path.extension().and_then(|ext| ext.to_str()) {
                    Some("plugin") => {
                        plugin_paths.insert(file_path.as_path().to_owned());
                    }
                    Some("cplugin") => {
                        compiled_plugin_paths.insert(file_path.as_path().to_owned());
                    }
                    _ => {}
                }
            }
        }

        for compiled_plugin_path in &compiled_plugin_paths {
            let plugin_path = compiled_plugin_path.with_extension("plugin");
            plugin_paths.remove(&plugin_path);
        }

        // Construct the wasm engine
        let mut config = Config::new();
        config.wasm_component_model(true);
        config.async_support(true);
        let engine = Engine::new(&config)?;

        let mut linker = Linker::new(&engine);
        add_to_linker_async(&mut linker)?;
        host::add_to_linker(&mut linker, |state: &mut PluginState| state.mut_host())?;
        let linker = Arc::new(linker);

        let mut plugins = vec![];

        // Non compiled plugins
        for plugin_path in plugin_paths {
            plugins.push(
                Plugin::load(linker.clone(), plugin_path)
                    .await
                    .context("load plugin")?,
            );
        }

        // Compiled plugins
        for compiled_plugin_path in compiled_plugin_paths {
            plugins.push(
                Plugin::load_compiled(linker.clone(), compiled_plugin_path)
                    .await
                    .context("load compiled plugin")?,
            );
        }

        Ok(Self { plugins })
    }

    #[allow(dead_code)]
    pub async fn plugin_names(&mut self) -> Result<Vec<String>> {
        let mut plugin_names = Vec::with_capacity(self.plugins.len());

        for plugin in &mut self.plugins {
            let plugin_name = plugin.name().await.context("get plugin name")?;

            plugin_names.push(plugin_name);
        }

        Ok(plugin_names)
    }

    pub async fn subcommands(&mut self) -> Result<Vec<String>> {
        let mut subcommands = Vec::with_capacity(self.plugins.len());

        for plugin in &mut self.plugins {
            let mut plugin_subcommands = plugin.subcommands().await.context("get plugin name")?;

            subcommands.append(&mut plugin_subcommands);
        }

        Ok(subcommands)
    }

    pub async fn run_sub_command(&mut self, subcommand: &str) -> Result<RunSubCommandOutcome> {
        for plugin in &mut self.plugins {
            let subcommands = plugin.subcommands().await.context("get subcommands")?;
            if subcommands.iter().any(|s| s == subcommand) {
                plugin.run().await.context("run subcommand")?;
                return Ok(RunSubCommandOutcome::Ok);
            }
        }

        Ok(RunSubCommandOutcome::NotFound)
    }
}

pub enum RunSubCommandOutcome {
    Ok,
    NotFound,
}
