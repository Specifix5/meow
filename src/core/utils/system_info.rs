use sysinfo::{ Pid, System };
use lazy_static::lazy_static;
use tokio::time::{ self, Duration, Instant };
use std::sync::{ Arc, RwLock };

lazy_static! {
  static ref SYSTEM_INFO: Arc<RwLock<System>> = {
    let info = System::new_all();
    Arc::new(RwLock::new(info))
  };

  static ref APP_UPTIME: Arc<RwLock<Instant>> = Arc::new(RwLock::new(Instant::now()));
}

pub async fn refresh_system_info() {
  loop {
    time::sleep(Duration::from_secs(5)).await;

    let mut sys_info = SYSTEM_INFO.write().unwrap();
    sys_info.refresh_all();
  }
}

pub fn get_memory_usage() -> u64 {
  let sys_info = SYSTEM_INFO.read().unwrap();
  let pid = std::process::id();
  let process = sys_info.process(Pid::from_u32(pid)).expect("Failed to get process info");
  let mem_usage = process.memory() / 1_000_000;

  mem_usage
}

pub fn get_app_uptime() -> u64 {
  let app_uptime = APP_UPTIME.read().unwrap();
  let uptime = app_uptime.elapsed().as_secs();
  uptime
}

pub fn get_app_uptime_string() -> String {
  let uptime_seconds = get_app_uptime();
  let days = uptime_seconds / 86400;
  let hours = (uptime_seconds % 86400) / 3600;
  let minutes = (uptime_seconds % 3600) / 60;
  let seconds = uptime_seconds % 60;

  format!("{}d {}h {}m {}s", days, hours, minutes, seconds)
}
