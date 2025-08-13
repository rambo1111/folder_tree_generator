// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashSet;
use tauri::api::dialog::blocking::FileDialogBuilder;

mod tree_generator;

// --- Default Ignore List ---
fn get_default_ignore_list() -> HashSet<String> {
    [
        ".git",
        ".vscode",
        "__pycache__",
        "node_modules",
        "venv",
        ".DS_Store",
        "target",
        "dist",
        "build",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}

// --- Tauri Commands ---

#[tauri::command]
fn get_initial_data() -> Vec<String> {
    let mut defaults: Vec<String> = get_default_ignore_list().into_iter().collect();
    defaults.sort();
    defaults // Only return the ignore list
}

#[tauri::command]
fn select_directory() -> Option<String> {
    FileDialogBuilder::new().pick_folder().map(|p| p.to_string_lossy().to_string())
}

#[tauri::command]
async fn generate_tree(path: String, ignore_list: Vec<String>) -> Result<String, String> {
    let ignore_set: HashSet<String> = ignore_list.into_iter().collect();
    tree_generator::generate_folder_tree(&path, &ignore_set)
}

// --- Main Function ---
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_initial_data,
            select_directory,
            generate_tree
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}