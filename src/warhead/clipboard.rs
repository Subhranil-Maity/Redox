use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use serde_json::json;
use Redox::utils::get_current_timestamp;


pub fn clip_map_to_json(clip: HashMap<String, String>) -> serde_json::Value {
    clip.iter().map(|(k, v)| format!("time:{} clip:{}", v, k)).collect::<Vec<String>>().join("\n").into()
}

pub fn start_clipboard(clips: Arc<Mutex<HashMap<String, String>>>) {
    //TODO! stop spawning new threads
    thread::spawn(move || {
        // let mut array_of_clips: Vec<String> = vec![];
        // let mut clips: HashMap<String, String>  = HashMap::new();
        loop {
            clipboard(&clips);
        }
    });
}

fn clipboard(clips: &Arc<Mutex<HashMap<String, String>>>) {
    thread::sleep(std::time::Duration::from_millis(100));
    let clipboard = clipboard_win::get_clipboard_string().unwrap();
    // clips.lock().unwrap().
    if clips.lock().unwrap().contains_key(&clipboard) { return; }
    clips.lock().unwrap().insert(clipboard.clone(), get_current_timestamp());
    println!("{}", clipboard);
}

fn clip_to_string(clip: HashMap<String, String>) -> String{
    clip.iter().map(|(k, v)| format!("time:{} clip:{}", v, k)).collect::<Vec<String>>().join("\n")
}