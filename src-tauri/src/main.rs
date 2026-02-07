#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

mod github;
mod godot;

#[derive(serde::Serialize)]
struct PullRequest {
    number: u32,
    title: String,
    html_url: String,
    updated_at: String,
    user: String,
    head_ref: String,
}

#[tauri::command]
async fn get_pull_requests() -> Result<Vec<PullRequest>, String> {
    github::fetch_prs("kinlly", "ylbtm")
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn launch_godot(pr_number: u32, branch_name: String) -> Result<String, String> {
    godot::launch(pr_number, &branch_name)
}

#[tauri::command]
fn launch_main() -> Result<String, String> {
    godot::launch_main()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_pull_requests,
            launch_godot,
            launch_main
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
