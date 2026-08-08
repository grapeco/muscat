use std::{ffi::OsStr, path::{Path, PathBuf}};

use hex_color::HexColor;
use iced::{Color, Element, Task, Theme, widget::{button, column, pick_list, text}};
use resolve_path::PathResolveExt;

use crate::func::{
    func::{Config, THEME_DIR, execute, list_dir, parse_config, parse_theme}, process::{restart, set_wallpaper}, traits::PathExt
};

#[derive(Clone)]
struct State {
    selected_file: Option<PathBuf>,
    status_messages: Vec<String>,
    current_theme: Theme,
    config: Config
}

#[derive(Clone)]
enum Message {
    FileSelected(PathBuf),
    Execute,
    PickFile,
    Error(Error)
}

#[derive(Clone)]
enum Error {
    PickerCancelled,
    ParseFailed,
    NoFile,
    ExecuteError,
}

fn hex_to_color(hex: &str) -> Color {
    let parsed = match hex.starts_with('#') {
        true => HexColor::parse_rgb(hex).unwrap_or(HexColor::default()),
        false => HexColor::parse_rgb(&format!("#{}", hex)).unwrap_or(HexColor::default())
    };
    
    return Color::from_rgb(
        parsed.r as f32 / 255.0, 
        parsed.g as f32 / 255.0, 
        parsed.b as f32 / 255.0,
    );
}

fn load_theme_from_file(filename: &Path) -> Result<Theme, Error> {
    match parse_theme(filename) {
        Ok(file) => {
            return Ok(
                Theme::custom(
                    file["scheme"].to_string(),
                    iced::theme::Palette {
                        background: hex_to_color(file["base00"].as_str().unwrap_or("000000")),
                        text: hex_to_color(file["base05"].as_str().unwrap_or("ffffff")),
                        primary: hex_to_color(file["base0D"].as_str().unwrap_or("0000ff")),
                        success: hex_to_color(file["base0B"].as_str().unwrap_or("00ff00")),
                        warning: hex_to_color(file["base0A"].as_str().unwrap_or("ffff00")),
                        danger: hex_to_color(file["base08"].as_str().unwrap_or("ff0000")),
                    }
                )   
            );
        }
        Err(_) => {
            return Err(Error::ParseFailed);
        }
    };
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::FileSelected(file) => {
            state.selected_file = Some(file.clone());
            
            match load_theme_from_file(&file) {
                Ok(theme) => state.current_theme = theme,
                Err(_) => return Task::done(Message::Error(Error::ParseFailed)),
            }

            return Task::none();
        }
        Message::PickFile => {
            return Task::perform(
                async {
                    rfd::FileDialog::new()
                        .set_directory("~".resolve())
                        .pick_file()
                },
                |file| match file {
                    Some(path) => Message::FileSelected(path),
                    None => Message::Error(Error::PickerCancelled),
                },
            );
        }
        Message::Error(e) => {
            match e {
                Error::PickerCancelled => state.status_messages.push("Failed to pick file\n".to_string()),
                Error::ParseFailed => state.status_messages.push("Failed to parse theme\n".to_string()),
                Error::NoFile => state.status_messages.push("Please, select your file\n".to_string()),
                Error::ExecuteError => state.status_messages.push("Failed to execute\n".to_string()),
            }

            return Task::none();
        }
        Message::Execute => {
            match state.selected_file.as_ref() {
                Some(file) => {
                    match execute(
                        &state.config.targets, 
                        file, 
                    ) {
                        Ok(_) => {
                            // Check if wallpapers option is set
                            if let Some(walls) = &state.config.wallpapers {
                                set_wallpaper(walls, &file.name_without_extension());
                            }
                        
                            // Check if restarts option is set
                            if let Some(rest) = &state.config.restarts {
                                restart(rest);
                            }
                        }
                        Err(_) => return Task::done(Message::Error(Error::ExecuteError))
                    }
                }
                None => return Task::done(Message::Error(Error::NoFile))
            }

            return Task::none();
        }
    }
}

fn view(state: &State) -> Element<'_, Message> {
    let path = Path::new(THEME_DIR);

    let themes: Vec<String> = match list_dir(&path) {
        Ok(f) => f.iter()
            .filter(|item| item.extension() == Some(OsStr::new("json")))
            .map(|p| p.name_without_extension())
            .collect(),
        Err(_) => vec![],
    };

    let show_name = state.selected_file.as_ref().map(|p| p.name_without_extension());
    
    column![
        pick_list(
            themes, 
            show_name, 
            move |s| Message::FileSelected(path.join(&s).with_extension("json"))
        )
            .placeholder("Select your favorite theme"),
        button(text("Process"))
            .on_press(Message::Execute),
        button(text("Pick file"))
            .on_press(Message::PickFile),
        text(state.status_messages.join(""))
    ].into()
}

pub fn gui() {        
    let mut status_message = vec![];
    let config: Config = match parse_config() {
        Ok(conf) => {
            status_message.push("Successfully load config file\n".to_string());
            conf
        }
        Err(e) => {
            status_message.push(format!("Config file not found, please create it:\n{}", e));
            Config::default()
        }
    };
    
    let init_state = State {
        selected_file: None,
        status_messages: status_message,
        current_theme: Theme::Dark,
        config,
    };
    
    iced::application(move || init_state.clone(), update, view)
        .theme(|state: &State| state.current_theme.clone())
        .run()
        .unwrap();
}