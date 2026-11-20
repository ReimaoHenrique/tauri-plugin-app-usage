use serde::{Deserialize, Serialize};

/// Uso de um aplicativo em primeiro plano. Tempos em milissegundos.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppUsage {
    pub package_name: String,
    pub app_name: String,
    /// Ícone PNG como data URI (`data:image/png;base64,...`), quando disponível.
    pub icon_base64: Option<String>,
    pub total_time_in_foreground: u64,
    pub today_time_in_foreground: u64,
    #[serde(rename = "last7DaysTimeInForeground")]
    pub last_7_days_time_in_foreground: u64,
    pub last_time_used: u64,
}

/// Ranking de apps + total de uso no dia.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageReport {
    pub apps: Vec<AppUsage>,
    pub total_today_ms: u64,
}

/// Resposta do comando Kotlin `queryUsage`.
#[cfg(mobile)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UsageResponse {
    pub apps: Vec<AppUsage>,
    pub total_today_ms: u64,
}

/// Resposta do comando Kotlin `hasPermission`.
#[cfg(mobile)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PermissionResponse {
    pub granted: bool,
}

/// Argumentos do comando Kotlin `queryUsage`.
#[cfg(mobile)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct QueryArgs {
    pub days: u16,
    pub package_name: Option<String>,
}
