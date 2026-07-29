use crate::func::func::{execute, parse_config};

// This function used for CLI mode
pub fn from_config() {
    let config = match parse_config() {
        Ok(conf) => conf,
        Err(e) => {
            eprintln!("Config error: {}", e);
            return;
        }
    }; 

    let targets = &config.targets;
    
    match execute(targets.to_vec(), &config.data, &config) {
        Ok(_) => {},
        Err(e) => eprintln!("Execution error: {}", e),
    }; 
}