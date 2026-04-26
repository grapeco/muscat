use std::{collections::HashMap, ffi::OsStr, fmt::Debug, process::Command};

use resolve_path::PathResolveExt;

fn run_command(args: Vec<&str>) {
    Command::new(args[0])
        .args(&args[1..])
        .spawn()
        .ok();
}

pub fn set_wallpaper(walls: Vec<HashMap<String, String>>, theme_name: String) {
    println!("{}", theme_name);
    for i in walls {
        if let Some(val) = i.get(&theme_name) {
            run_command(vec!["awww", "img", val.resolve().to_str().unwrap()]);
        }
    }
}

pub fn kill_process(process: &str) {
    println!("🔪 Завершаем процесс: {:?}", process);
    
    run_command(vec!["pkill", process]);
}

pub fn start_process(process: &str) {
    println!("Запускаем процесс: {:?}", process);
        
    run_command(vec![process]);
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