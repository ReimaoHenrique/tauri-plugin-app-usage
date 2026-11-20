use tauri::{AppHandle, command, Runtime};

use crate::{AppUsageExt, Result};

/// Verifica se o usuário concedeu "Acesso aos dados de uso" no Android.
#[command]
pub(crate) fn check_permission<R: Runtime>(app: AppHandle<R>) -> Result<bool> {
    app.app_usage().check_permission()
}

/// Abre as configurações do sistema. A permissão é concedida pelo usuário.
#[command]
pub(crate) fn request_permission<R: Runtime>(app: AppHandle<R>) -> Result<bool> {
    app.app_usage().request_permission()
}

/// Consulta todos os apps com uso registrado, já ordenados como ranking.
#[command]
pub(crate) fn get_all_app_usage<R: Runtime>(app: AppHandle<R>, days: u16) -> Result<crate::UsageReport> {
    app.app_usage().get_all_app_usage(days)
}

/// Consulta um único app pelo package name.
#[command]
pub(crate) fn get_app_usage<R: Runtime>(
    app: AppHandle<R>,
    package_name: String,
    days: u16,
) -> Result<Option<crate::AppUsage>> {
    app.app_usage().get_app_usage(&package_name, days)
}
