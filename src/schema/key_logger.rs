use crate::utils::get_current_timestamp;

pub struct KeyLoggData {
    pub file_name: String,
    pub title: String,
    pub data: String,
    pub timestamp: String,
}

impl KeyLoggData {
    pub fn new(file_name: String, title: String, data: String) -> KeyLoggData {
        KeyLoggData {
            file_name,
            title,
            data,
            timestamp: get_current_timestamp(),
        }
    }
    pub fn to_string(&self) -> String {
        format!("time:{} filename:{} title:{} data:{}", self.timestamp, self.file_name, self.title, self.data)
    }
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "time": self.timestamp,
            "filename": self.file_name,
            "title": self.title,
            "data": self.data
        })
    }
}

