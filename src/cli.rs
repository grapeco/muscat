use std::path::Path;

use crate::func::{func::{THEME_DIR, execute, init, parse_config}, process::{restart, set_wallpaper}, traits::PathExt};

// This function used for CLI mode
pub fn from_config() {
    if let Err(e) = init() {
        eprintln!("Failed to create all needed files, Please create them by youyrself\n{}", e);
    }
    
    let config = match parse_config() {
        Ok(conf) => conf,
        Err(e) => {
            eprintln!("Config error: {}", e);
            return;
        }
    }; 

    let theme_path = &Path::new(THEME_DIR).join(&config.theme).with_extension("json");
    let targets: Vec<&Path> = config.targets.iter().map(|p| p.as_path()).collect();

    match execute(&targets, theme_path) {
        Ok(_) => {
            // Check if wallpapers option is set
            if let Some(walls) = &config.wallpapers {
                set_wallpaper(walls, config.theme.name_without_extension());
            }
        
            // Check if restarts option is set
            if let Some(rest) = &config.restarts {
                restart(rest);
            }
        }
        Err(e) => eprintln!("Execution error: {}", e),
    }; 
}