use std::{
    fmt::Debug, fs::{self}, path::{Path, PathBuf}, thread::sleep, time::Duration, 
};

use mustache;
use serde::Deserialize;
use serde_json::Value;
use resolve_path::PathResolveExt;

use super::process;

// pub const PATH_TO_CONFIG: &str = "~/.config/muscat/config.jsonc";
pub const PATH_TO_CONFIG: &str = "config.jsonc";

#[derive(Debug, Deserialize)]
pub struct Config {
    data: String,
    pub targets: Vec<String>,
    restarts: Option<Vec<String>>,
}

pub fn list_dir<T: AsRef<Path>>(dir: T) -> Vec<String> {
    let directory = fs::read_dir(dir).unwrap();
    let mut string_dir: Vec<String> = Vec::new();
    
    for entry in directory {
        let entry = entry.unwrap().file_name();
        string_dir.push(entry.into_string().unwrap());
    }
    
    return string_dir;
}

pub fn execute<T>(paths: Vec<T>, data: T)
where 
    T: AsRef<Path> + Clone,
    PathBuf: From<T>
{
    let data_content: Value = json5::from_str::<Value>(
        fs::read_to_string(data).unwrap().as_str()
    ).expect("Can't parse data file");

    for file in paths {    
        let mut name = PathBuf::from(file.clone());
        name.set_extension("");
        
        // This code founds templates by target file name
        // Example: style.css(target) - style-temp.css(template)
        let template_file_content = fs::read_to_string(
            format!(
                "{}-temp{}", 
                name.display(),
                match PathBuf::from(file.clone()).extension() {
                    Some(ext) => format!(".{}", ext.display()),
                    None => format!(""),
                }
            )
        ).expect("Can't read template file");
        
        let template = mustache::compile_str(&template_file_content).expect("Can't compile str");

        // Writing compiled mustache template into target file
        let target = template.render_to_string(&data_content).expect("Can't render");
        fs::write(file, target).expect("No such file");
    }
}

pub fn parse_config() -> Config {
    let config_content = fs::read_to_string(PATH_TO_CONFIG.resolve()).expect("No such file");    
    let config: Config = json5::from_str(&config_content).unwrap();
    
    return config;
}

pub fn from_config() {
    let config = parse_config();
    let targets = config.targets
        .iter()
        .map(|target| target.resolve())
        .collect();
    
    execute(targets, config.data.resolve());
    
    restart(); 
    

}

pub fn restart() {
    let restarts = match parse_config().restarts {
        Some(vec) => vec,
        None => vec![]
    };
    
    // Iterating in list of restarts
    for i in restarts {
        println!("{}", &i);
        
        if process::check_valid(&i) == true {
            process::kill_process(&i);
            
            sleep(Duration::from_millis(100));
            
            let start_name = match i.trim() {
                "zed" => "zeditor",
                other => other
            };
            process::start_process(start_name);
        }
    }
}