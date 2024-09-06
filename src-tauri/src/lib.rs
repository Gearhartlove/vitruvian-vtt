use serde_json::Value;
use vitruvian_types::prelude::*;

pub mod ingestion;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Test function for testing sending data to the frontend
#[tauri::command]
fn get_test_data() -> Vec<Entity> {
    let mut entity : Entity = Entity::new();
    entity.add(Name("Test".to_string()));
    vec![entity]
}

#[tauri::command]
fn set_test_data(data : Value) {
    let mut entity : Entity = Entity::new();
    let name : Name = serde_json::from_value(data["Name"].clone()).unwrap();
    let damage : Damage = serde_json::from_value(data["Damage"].clone()).unwrap();
    entity.add(name);
    entity.add(damage);
    println!("Test {}", entity.to_string());
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, get_test_data, set_test_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
