use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::AppUsagePlugin;
#[cfg(mobile)]
use mobile::AppUsagePlugin;

/// Extensões para acessar a API do plugin a partir de `App`, `AppHandle` e `Window`.
pub trait AppUsageExt<R: Runtime> {
    fn app_usage(&self) -> &AppUsagePlugin<R>;
}

impl<R: Runtime, T: Manager<R>> crate::AppUsageExt<R> for T {
    fn app_usage(&self) -> &AppUsagePlugin<R> {
        self.state::<AppUsagePlugin<R>>().inner()
    }
}

/// Inicializa o plugin `app-usage`.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("app-usage")
        .invoke_handler(tauri::generate_handler![
            commands::check_permission,
            commands::request_permission,
            commands::get_all_app_usage,
            commands::get_app_usage
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let app_usage = mobile::init(app, api)?;
            #[cfg(desktop)]
            let app_usage = desktop::init(app, api)?;
            app.manage(app_usage);
            Ok(())
        })
        .build()
}
