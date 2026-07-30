use std::path::Path;

use crate::func::{func::{execute, parse_config}, process::{restart, set_wallpaper}, traits::PathExt};

// This function used for CLI mode
pub fn from_config() {
    let config = match parse_config() {
        Ok(conf) => conf,
        Err(e) => {
            eprintln!("Config error: {}", e);
            return;
        }
    }; 

    let targets: Vec<&Path> = config.targets.iter().map(|p| p.as_path()).collect();

    match execute(&targets, &config.data) {
        Ok(_) => {
            // Check if wallpapers option is set
            if let Some(walls) = &config.wallpapers {
                set_wallpaper(walls, config.data.name_without_extension());
            }
        
            // Check if restarts option is set
            if let Some(rest) = &config.restarts {
                restart(rest);
            }
        }
        Err(e) => eprintln!("Execution error: {}", e),
    }; 
}