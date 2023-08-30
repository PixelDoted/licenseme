//! Uses the 'spdx.org' api to find licenses

mod result;
mod typ;

pub use result::*;
use std::path::PathBuf;
pub use typ::*;

use crate::util::get_cache;

// Load List
pub fn load_list() -> anyhow::Result<SPDXList> {
    let cache_dir = get_cache();
    let cache_list = cache_dir.join("list.json");
    Ok(if !cache_list.exists() {
        std::fs::create_dir_all(cache_dir)?;
        download_list(cache_list)?
    } else {
        let data = std::fs::read(cache_list)?;
        serde_json::from_slice(&data)?
    })
}

// ---- Download List ----
fn download_list(path: PathBuf) -> anyhow::Result<SPDXList> {
    const URL: &str = "https://spdx.org/licenses/licenses.json";
    let response = reqwest::blocking::get(URL)?;
    let list: SPDXList = response.json()?;

    std::fs::write(path, serde_json::to_vec(&list)?)?;
    Ok(list)
}

// ---- Load License ----
pub fn load_license(spdx_id: &str) -> anyhow::Result<String> {
    let path = get_cache().join(spdx_id);
    if path.exists() {
        let bytes = std::fs::read(path)?;
        Ok(String::from_utf8(bytes)?)
    } else {
        download_license(spdx_id, path)
    }
}

// ---- Download License ----
fn download_license(spdx_id: &str, path: PathBuf) -> anyhow::Result<String> {
    let url = format!("https://spdx.org/licenses/{}.json", spdx_id);
    let response = reqwest::blocking::get(url)?;
    let json: serde_json::Value = response.json()?;

    // Decode
    let root = json.as_object().unwrap();
    let value = root.get("licenseText").unwrap();
    let text = value.as_str().unwrap().to_string();

    // Save
    std::fs::write(path, &text)?;
    Ok(text)
}
