use std::{
    collections::HashMap, fs::{self}, path::{Path, PathBuf}, thread::sleep, time::Duration 
};

use mustache;
use serde::{Deserialize};
use serde_json::Value;
use resolve_path::PathResolveExt;

use crate::func::process;

//pub const PATH_TO_CONFIG: &str = "~/.config/muscat/config.jsonc";
pub const PATH_TO_CONFIG: &str = "config.jsonc";

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    data: String,
    pub data_dir: Option<PathBuf>,
    pub targets: Vec<String>,
    pub wallpapers: Option<Vec<HashMap<String, String>>>,
    restarts: Option<Vec<String>>,
}

// FOR FUTURE, maybe

// #[derive(Default, Clone, Serialize, Deserialize, Debug)]
// #[allow(non_snake_case)]
// pub struct Base16 {
//     pub base00: String,
//     pub base01: String,
//     pub base02: String,
//     pub base03: String,
//     pub base04: String,
//     pub base05: String,
//     pub base06: String,
//     pub base07: String,
//     pub base08: String,
//     pub base09: String,
//     pub base0A: String,
//     pub base0B: String,
//     pub base0C: String,
//     pub base0D: String,
//     pub base0F: String,
// }

pub fn list_dir<T: AsRef<Path>>(dir: T) -> Vec<PathBuf> {
    let directory = fs::read_dir(dir).unwrap();
    let mut string_dir: Vec<PathBuf> = Vec::new();
    
    for entry in directory {
        let entry = entry.unwrap();
        string_dir.push(entry.path());
    }
    
    return string_dir;
}

pub fn execute<T>(paths: Vec<T>, data_path: T)
where 
    T: AsRef<Path> + Clone,
    PathBuf: From<T>
{
    let data_content = parse_theme(data_path.into());

    for file in paths {    
        let name = PathBuf::from(file.clone()).with_extension("");
        
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

pub fn parse_theme(data_file: PathBuf) -> Value {
    json5::from_str(
        fs::read_to_string(&data_file.resolve()).unwrap().as_str()
    ).expect("Can't parse data file")
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
            
            sleep(Duration::from_millis(300));
            
            let start_name = match i.trim() {
                "zed" => "zeditor",
                other => other
            };
            process::start_process(start_name);
        }
    }
}