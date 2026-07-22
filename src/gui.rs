use std::{ffi::OsStr, path::PathBuf};

use hex_color::HexColor;
use iced::{Color, Element, Theme, widget::{button, column, pick_list, text}};
use resolve_path::PathResolveExt;

use crate::func::{
    func::{Config, execute, list_dir, parse_config, parse_theme}, 
    traits::{PathBufExt}
};

#[derive(Clone)]
struct State {
    selected_file: Option<PathBuf>,
    status_message: String,
    current_theme: Theme,
    config: Config
}

#[derive(Clone)]
enum Message {
    FileSelected(PathBuf),
    Execute,
    PickFile,
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

fn load_theme_from_file(filename: PathBuf, state: &mut State) -> Theme {
    println!("{:?}", filename);
    let theme = match parse_theme(filename.clone()) {
        Ok(file) => {
            state.status_message = format!("File: {:?} is found", filename);
            Theme::custom(
                file["name"].as_str().unwrap_or("").to_owned(),
                iced::theme::Palette {
                    background: hex_to_color(file["base00"].as_str().unwrap_or("000000")),
                    text: hex_to_color(file["base05"].as_str().unwrap_or("ffffff")),
                    primary: hex_to_color(file["base0D"].as_str().unwrap_or("0000ff")),
                    success: hex_to_color(file["base0B"].as_str().unwrap_or("00ff00")),
                    warning: hex_to_color(file["base0A"].as_str().unwrap_or("ffff00")),
                    danger: hex_to_color(file["base08"].as_str().unwrap_or("ff0000")),
                }
            )
        }
        Err(e) => {
            state.status_message = format!("Failed to parse theme:\n{}", e);
            Theme::Dark
        }
    };

    return theme;
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::FileSelected(file) => {
            state.selected_file = Some(file.clone());
            state.current_theme = load_theme_from_file(file, state);
        }
        Message::PickFile => {
            let file = rfd::FileDialog::new()
                .set_directory("~".resolve())
                .pick_file();
            
            if let Some(f) = file {
                update(state, Message::FileSelected(f));
            }
        }
        Message::Execute => {
            match state.selected_file.as_ref() {
                Some(file) => {
                    if let Err(e) = execute(
                        state.config.targets.to_owned(), 
                        file.to_owned(), 
                        state.config.to_owned()
                    ) {
                        state.status_message = format!("Failed to execute:\n{}", e);
                    }
                }
                None => state.status_message = "Please, select your file".to_string(),
            }
        }
    }
}

fn view(state: &State) -> Element<'_, Message> {
    let path = match &state.config.data_dir {
        Some(p) => p.resolve(),
        None => "~/.config/muscat/themes/".resolve()
    };    
    
    let files: Vec<String> = list_dir(&path)
        .into_iter()
        .filter(|item| item.extension().unwrap_or(OsStr::new("")) == "json")
        .map(|p| p.name_without_extension())
        .collect();

    let show_name = state.selected_file.as_ref().map(|path| path.name_without_extension());
    
    column![
        pick_list(
            files, 
            show_name, 
            move |s| Message::FileSelected(path.join(&s).with_extension("json").resolve().to_path_buf())
        )
            .placeholder("Select your favorite theme"),
        button(text("Process"))
            .on_press(Message::Execute),
        button(text("Pick file"))
            .on_press(Message::PickFile),
        text(format!("{}", state.status_message))
    ].into()
}

pub fn gui() {        
    let status_message;
    let config: Config = match parse_config() {
        Ok(conf) => {
            status_message = "Successfully load config file".to_string();
            conf
        }
        Err(e) => {
            status_message = format!("Config file not found, please create it:\n{}", e);
            Config::default()
        }
    };
    
    let init_state = State {
        selected_file: None,
        status_message,
        current_theme: Theme::Dark,
        config,
    };
    
    iced::application(move || init_state.clone(), update, view)
        .theme(|state: &State| state.current_theme.clone())
        .run()
        .unwrap();
}