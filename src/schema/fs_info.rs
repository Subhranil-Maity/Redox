use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct ItemInfo {
    name: String,
    path: String,
    size: u64,
    is_symlink: bool,
    is_folder: bool,
}
impl ItemInfo {
    pub fn new(name: String, path: String, size: u64, is_symlink: bool, is_folder: bool) -> ItemInfo {
        ItemInfo {
            name,
            path,
            size,
            is_symlink,
            is_folder,
        }
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn from_json(json: &str) -> ItemInfo {
        serde_json::from_str(json).unwrap()
    }
}