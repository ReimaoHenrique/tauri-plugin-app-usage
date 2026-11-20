const COMMANDS: &[&str] = &[
    "check_permission",
    "request_permission",
    "get_all_app_usage",
    "get_app_usage",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .build();
}
