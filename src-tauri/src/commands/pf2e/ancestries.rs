use std::fs;

use crate::ingestion::pf2e::ancestry::Ancestry;

#[tauri::command]
pub fn get_ancestries() -> Result<String, String> {
    let ancestry_file = "data/pf2e/packs/ancestries/dwarf.json";

    let ancestry =
        fs::read_to_string(ancestry_file).map_err(|e| format!("failed to read file: {}", e))?;

    let ancestry: Ancestry =
        serde_json::from_str(&ancestry).map_err(|e| format!("failed to parse json: {}", e))?;

    Ok(format!("{:?}", ancestry))
}
