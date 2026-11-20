use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::{AppUsage, PermissionResponse, QueryArgs, Result, UsageReport, UsageResponse};

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_app_usage);

// Inicializa a classe Kotlin no Android (e a classe Swift no iOS, se existir).
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> Result<AppUsagePlugin<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("com.plugin.app_usage", "UsagePlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_app_usage)?;
    Ok(AppUsagePlugin(handle))
}

/// Acesso às APIs nativas de uso de aplicativos.
pub struct AppUsagePlugin<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> AppUsagePlugin<R> {
    pub fn check_permission(&self) -> Result<bool> {
        let response: PermissionResponse = self.0.run_mobile_plugin("hasPermission", ())?;
        Ok(response.granted)
    }

    pub fn request_permission(&self) -> Result<bool> {
        self.0.run_mobile_plugin::<()>("openUsageSettings", ())?;
        Ok(true)
    }

    pub fn get_all_app_usage(&self, days: u16) -> Result<UsageReport> {
        let response: UsageResponse = self.0.run_mobile_plugin(
            "queryUsage",
            QueryArgs {
                days: days.max(1).min(365),
                package_name: None,
            },
        )?;
        Ok(UsageReport {
            apps: response.apps,
            total_today_ms: response.total_today_ms,
        })
    }

    pub fn get_app_usage(&self, package_name: &str, days: u16) -> Result<Option<AppUsage>> {
        let response: UsageResponse = self.0.run_mobile_plugin(
            "queryUsage",
            QueryArgs {
                days: days.max(1).min(365),
                package_name: Some(package_name.to_string()),
            },
        )?;
        Ok(response.apps.into_iter().next())
    }
}
