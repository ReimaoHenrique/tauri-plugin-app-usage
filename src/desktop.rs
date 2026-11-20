use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::{Error, Result, UsageReport};

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> Result<AppUsagePlugin<R>> {
    Ok(AppUsagePlugin(app.clone()))
}

/// No desktop não existe `UsageStatsManager`: os comandos retornam `Unsupported`.
pub struct AppUsagePlugin<R: Runtime>(AppHandle<R>);

impl<R: Runtime> AppUsagePlugin<R> {
    pub fn check_permission(&self) -> Result<bool> {
        Err(Error::Unsupported)
    }

    pub fn request_permission(&self) -> Result<bool> {
        Err(Error::Unsupported)
    }

    pub fn get_all_app_usage(&self, _days: u16) -> Result<UsageReport> {
        Err(Error::Unsupported)
    }

    pub fn get_app_usage(&self, _package_name: &str, _days: u16) -> Result<Option<crate::AppUsage>> {
        Err(Error::Unsupported)
    }
}
