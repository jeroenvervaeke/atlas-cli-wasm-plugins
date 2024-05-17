use super::adapter::adapt_wasm_bytes;
use super::component::atlascli::plugin::host::Host;
use anyhow::{Context, Result};
use std::path::Path;
use std::sync::Arc;
use tokio::fs::write;
use wasmtime::component::{Component, Linker};
use wasmtime::Store;
use wasmtime_wasi::preview1::WasiP1Ctx;
use wasmtime_wasi::{WasiCtxBuilder, WasiView};

pub struct Plugin {
    component: Component,
    linker: Arc<Linker<PluginState>>,
    store: Store<PluginState>,
}

impl Plugin {
    fn new(
        component: Component,
        linker: Arc<Linker<PluginState>>,
        store: Store<PluginState>,
    ) -> Self {
        Self {
            component,
            linker,
            store,
        }
    }

    pub async fn load(linker: Arc<Linker<PluginState>>, path: impl AsRef<Path>) -> Result<Self> {
        let component = Self::load_component_and_compile(&linker, path)
            .await
            .context("load component and compile")?;

        let store = Self::build_store(&linker);

        Ok(Self::new(component, linker, store))
    }

    pub async fn load_compiled(
        linker: Arc<Linker<PluginState>>,
        path: impl AsRef<Path>,
    ) -> Result<Self> {
        let component =
            Self::load_compiled_component(&linker, path).context("load compiled component")?;

        let store = Self::build_store(&linker);

        Ok(Self::new(component, linker, store))
    }

    async fn load_component_and_compile(
        linker: &Linker<PluginState>,
        path: impl AsRef<Path>,
    ) -> Result<Component> {
        let adapted_wasm: Vec<u8> = adapt_wasm_bytes(&path).await.context("adapting wasm")?;

        let component = Component::from_binary(&linker.engine(), &adapted_wasm)?;
        let serialized_plugin = component.serialize().context("serializing plugin")?;
        write(path.as_ref().with_extension("cplugin"), &serialized_plugin)
            .await
            .context("saving serialized plugin")?;

        Ok(component)
    }

    fn load_compiled_component(
        linker: &Linker<PluginState>,
        path: impl AsRef<Path>,
    ) -> Result<Component> {
        // safety only load cplugin, compiled by this CLI, unless you want to see funky things happen
        let component = unsafe { Component::deserialize_file(&linker.engine(), path)? };
        Ok(component)
    }

    fn build_store(linker: &Linker<PluginState>) -> Store<PluginState> {
        Store::new(
            &linker.engine(),
            PluginState {
                host: PluginHost {
                    bearer_token: "example-bearer".into(),
                },
                wasi_ctx: WasiCtxBuilder::new()
                    .inherit_args()
                    .inherit_env()
                    .inherit_stdio()
                    .build_p1(),
            },
        )
    }

    async fn create_plugin_instance(&mut self) -> Result<super::component::Plugin> {
        let (plugin, _instance) = super::component::Plugin::instantiate_async(
            &mut self.store,
            &self.component,
            &self.linker,
        )
        .await
        .context("instantiate_async")?;

        Ok(plugin)
    }

    pub async fn name(&mut self) -> Result<String> {
        let plugin = self
            .create_plugin_instance()
            .await
            .context("create plugin instance")?;
        plugin
            .atlascli_plugin_info()
            .call_name(&mut self.store)
            .await
    }

    pub async fn subcommands(&mut self) -> Result<Vec<String>> {
        let plugin = self
            .create_plugin_instance()
            .await
            .context("create plugin instance")?;

        plugin
            .atlascli_plugin_info()
            .call_sub_commands(&mut self.store)
            .await
            .context("call sub_commands")
    }

    pub async fn run(&mut self) -> Result<()> {
        let plugin = self
            .create_plugin_instance()
            .await
            .context("create plugin instance")?;

        // Will restul on err on panic, which is fine
        let _invoke_result = plugin
            .atlascli_plugin_info()
            .call_run(&mut self.store)
            .await;

        Ok(())
    }
}

pub struct PluginHost {
    bearer_token: String,
}

// Implmentation of the host interface defined in the wit file.
#[async_trait::async_trait]
impl Host for PluginHost {
    async fn bearer_token(&mut self) -> wasmtime::Result<String> {
        Ok(self.bearer_token.clone())
    }
}

pub struct PluginState {
    host: PluginHost,
    wasi_ctx: WasiP1Ctx,
}

impl WasiView for PluginState {
    fn table(&mut self) -> &mut wasmtime_wasi::ResourceTable {
        self.wasi_ctx.table()
    }

    fn ctx(&mut self) -> &mut wasmtime_wasi::WasiCtx {
        self.wasi_ctx.ctx()
    }
}

impl PluginState {
    pub fn mut_host(&mut self) -> &mut PluginHost {
        &mut self.host
    }
}
