use std::{ffi::OsStr, fmt::Debug, process::Command};

pub fn kill_process<T: AsRef<OsStr> + Debug>(process: T) {
    println!("🔪 Завершаем процесс: {:?}", process);
    
    Command::new("pkill")
        .arg(process)
        .status()
        .ok();
}

pub fn start_process<T: AsRef<OsStr> + Debug>(process: T) {
    println!("Запускаем процесс: {:?}", process);
        
    Command::new(process)
        .spawn()
        .ok();
}

pub fn check_valid<T: AsRef<OsStr> + Debug>(process: T) -> bool {
    let output = Command::new("pgrep")
        .arg(process)
        .output();
    
    match output {
        Ok(val) if !val.stdout.is_empty() => return true,
        Ok(_) => return false,
        Err(_) => return false
    }
}