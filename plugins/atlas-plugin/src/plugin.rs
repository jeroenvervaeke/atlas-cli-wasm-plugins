#[allow(unused_imports)]
pub use atlascli::plugin::host;
use exports::atlascli::plugin::info::Guest;

use crate::plugin_main;

wit_bindgen::generate!({
    path: "../../wit/plugin.wit",
    world: "plugin"
});

const PLUGIN_NAME: &str = "atlas-plugin";
struct Plugin;

export!(Plugin);

impl Guest for Plugin {
    fn name() -> String {
        PLUGIN_NAME.to_string()
    }

    fn run() -> Result<(), String> {
        plugin_main().map_err(|e| e.to_string())?;
        Ok(())
    }

    fn sub_commands() -> Vec<String> {
        vec![
            "accessLists".to_string(),
            "accessLogs".to_string(),
            "alerts".to_string(),
            "auditing".to_string(),
            "auth".to_string(),
            "backups".to_string(),
            "cloudProviders".to_string(),
            "clusters".to_string(),
            "config".to_string(),
            "customDbRoles".to_string(),
            "customDns".to_string(),
            "dataFederation".to_string(),
            "dataLakePipelines".to_string(),
            "dbusers".to_string(),
            "deployments".to_string(),
            "events".to_string(),
            "federatedAuthentication".to_string(),
            "integrations".to_string(),
            "kubernetes".to_string(),
            "liveMigrations".to_string(),
            "logs".to_string(),
            "maintenanceWindows".to_string(),
            "metrics".to_string(),
            "networking".to_string(),
            "organizations".to_string(),
            "performanceAdvisor".to_string(),
            "privateEndpoints".to_string(),
            "processes".to_string(),
            "projects".to_string(),
            "security".to_string(),
            "serverless".to_string(),
            "setup".to_string(),
            "streams".to_string(),
            "teams".to_string(),
            "users".to_string(),
        ]
    }
}