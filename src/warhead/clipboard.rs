use std::collections::HashMap;
use std::thread;
use crate::utils::get_current_timestamp;

pub fn start_clipboard() {
    //TODO! stop spawning new threads
    thread::spawn(|| {
        // let mut array_of_clips: Vec<String> = vec![];
        let mut clips: HashMap<String, String>  = HashMap::new();
        loop {
            clipboard(&mut clips);
        }
    });
}

fn clipboard(clips: &mut HashMap<String, String>) {
    thread::sleep(std::time::Duration::from_millis(100));
    let clipboard = clipboard_win::get_clipboard_string().unwrap();
    if clips.contains_key(&clipboard) { return; }
    clips.insert(clipboard.clone(), get_current_timestamp());
    println!("{}", clipboard);
}

fn clip_to_string(clip: HashMap<String, String>) -> String{
    clip.iter().map(|(k, v)| format!("time:{} clip:{}", v, k)).collect::<Vec<String>>().join("\n")
}