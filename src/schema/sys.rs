use serde::{Deserialize, Serialize};
use sysinfo::{Disk, DiskKind, Disks, System};

#[derive(Deserialize, Serialize, Debug)]
pub struct Drive {
    disk_name: String,
    file_system_format: String,
    disk_type: String,
    removable: bool,
    path: String,
    total_space: u64,
    used_space: u64,
}
impl Drive {
    pub fn new(disk: &Disk) -> Drive {
        Drive {
            disk_name: disk.name().to_string_lossy().to_string(),
            file_system_format: disk.file_system().to_str().unwrap_or("UNKNOWN").to_string(),
            disk_type: match disk.kind() {
                DiskKind::HDD => "HDD".to_string(),
                DiskKind::SSD => "SSD".to_string(),
                _ => "UNKNOWN".to_string(),
            },
            removable: disk.is_removable(),
            path: disk.mount_point().to_str().unwrap().to_string(),
            total_space: disk.total_space(),
            used_space: disk.available_space(),
        }
    }
}

pub fn get_drive_arr() -> Vec<Drive> {
    let n =  Disks::new_with_refreshed_list();
    let mut ret: Vec<Drive> = Vec::new();
    for m in &n {
        ret.push(Drive::new(m));
    }
    ret
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SysInfo {
    device_name: String,
    platform: String,
    version: String,
    drive: Vec<Drive>,
}
static mut SYSTEM: Option<System> = None;
pub fn get_system() -> &'static mut System {
    let f = unsafe { SYSTEM.get_or_insert(System::new()) };
    f
}
impl SysInfo {
    pub fn get() -> SysInfo {
        // let system = get_system();
        // system.refresh_system(); 
        SysInfo {
            device_name: sysinfo::System::host_name().unwrap_or("".to_string()),
            platform: sysinfo::System::name().unwrap_or("".to_string()),
            version: sysinfo::System::os_version().unwrap_or("".to_string()),
            drive: get_drive_arr(),
        }
    }
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "device_name": self.device_name,
            "platform": self.platform,
            "version": self.version,
            "drive": self.drive
        })
    }
    pub fn from_json(json: &str) -> SysInfo {
        serde_json::from_str(json).unwrap()
    }
}

