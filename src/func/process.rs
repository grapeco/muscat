use std::{
    collections::HashMap, process::{Command}, thread::sleep, time::Duration 
};

use resolve_path::PathResolveExt;

fn run_command(args: &[&str]) {
    Command::new(args[0])
        .args(&args[1..])
        .spawn()
        .expect("Can't run command");
}

pub fn set_wallpaper(walls: &[HashMap<String, String>], theme_name: &str) {
    for i in walls {
        if let Some(val) = i.get(theme_name) {
            run_command(&["awww", "img", val.resolve().to_str().unwrap()]);
        }
    }
}

pub fn kill_process(process: &str) {
    // println!("Killing process: {:?}", process);
    
    run_command(&["pkill", process]);
}

pub fn start_process(process: &[&str]) {
    // println!("Starting process: {:?}", process);
        
    run_command(process);
}

// Check if process exists
pub fn check_valid(process: &str) -> bool {
    // pgrep returns PID of process
    let output = Command::new("pgrep")
        .arg(process)
        .output();

    // If pgrep returns nothing, then process don't exist
    match output {
        Ok(val) if !val.stdout.is_empty() => return true,
        Ok(_) => return false,
        Err(_) => return false
    }
}

pub fn restart(restarts: &[String]) {
    // Iterating in list of restarts
    for name in restarts {
        if check_valid(&name) == true {
            kill_process(&name);

            // Avoid race condition
            sleep(Duration::from_millis(300));
            
            let start_name: &[&str] = match name.trim() {
                "zed" => &["zeditor", "."],
                other => &[other],
            };
            
            start_process(start_name);
        }
    }
}