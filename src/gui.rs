use std::{path::PathBuf};

use hex_color::HexColor;
use iced::{Color, Element, Theme, widget::{button, column, pick_list, text}};
use resolve_path::PathResolveExt;

use crate::func::{
    func::{self, Config, execute, list_dir, parse_config, parse_theme}, 
    process::set_wallpaper,
    traits::{PathBufExt, StringExt}
};

#[derive(Clone)]
struct State {
    selected_file: Option<PathBuf>,
    current_theme: Theme,
    config: Config
}

#[derive(Clone)]
enum Message {
    FileSelected(String),
    Execute,
    PickFile,
}

fn hex_to_color(hex: &str) -> Color {
    let parsed = match hex.starts_with('#') {
        true => HexColor::parse_rgb(hex).unwrap(),
        false => HexColor::parse_rgb(&format!("#{}", hex)).unwrap()
    };
    
    return Color::from_rgb(
        parsed.r as f32 / 255.0, 
        parsed.g as f32 / 255.0, 
        parsed.b as f32 / 255.0,
    );
}

fn load_theme_from_file(filename: PathBuf) -> Theme {
    let theme_file = parse_theme(filename);
        
    let theme = Theme::custom(
        theme_file["name"].as_str().unwrap_or("").to_owned(),
        iced::theme::Palette {
            background: hex_to_color(theme_file["base00"].as_str().unwrap_or("000000")),
            text: hex_to_color(theme_file["base05"].as_str().unwrap_or("ffffff")),
            primary: hex_to_color(theme_file["base0D"].as_str().unwrap_or("0000ff")),
            success: hex_to_color(theme_file["base0B"].as_str().unwrap_or("00ff00")),
            warning: hex_to_color(theme_file["base0A"].as_str().unwrap_or("ffff00")),
            danger: hex_to_color(theme_file["base08"].as_str().unwrap_or("ff0000")),
        }
    );
    
    return theme;
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::FileSelected(file) => {
            state.selected_file = Some(file.to_path_buf().with_extension("json"));
            
            let path = match &state.config.data_dir {
                Some(p) => p.join(state.selected_file.clone().unwrap())
                    .resolve()
                    .to_path_buf(),
                None => PathBuf::from("~/.config/muscat/themes/")
                    .join(state.selected_file.clone().unwrap())
                    .resolve()
                    .to_path_buf(),
            };
            
            state.current_theme = load_theme_from_file(path);
        }
        Message::PickFile => {
            let file = rfd::FileDialog::new()
                .set_directory("~".resolve())
                .pick_file();
            
            match file {
                Some(f) => update(state, Message::FileSelected(f.display().to_string())),
                None => update(state, Message::FileSelected("".to_string()))
            }
        }
        Message::Execute => {
            let targets = parse_config().targets
                .iter()
                .map(|target| target.resolve().to_path_buf())
                .collect();
            
            let path = match &state.config.data_dir {
                Some(p) => p.resolve().join(state.selected_file.clone().unwrap()),
                None => PathBuf::from("~/.config/muscat/themes/".resolve()).join(state.selected_file.as_ref().unwrap())
            };            
            
            execute(targets, path);
            
            if let Some(walls) = &mut state.config.wallpapers {
                set_wallpaper(walls.to_owned(), state.selected_file.as_ref().unwrap().name_without_extension());
            }
            
            func::restart();
        }
    }
}

fn view(state: &State) -> Element<'_, Message> {
    let path = match &state.config.data_dir {
        Some(p) => p.resolve(),
        None => "~/.config/muscat/themes/".resolve()
    };    
    
    let files: Vec<String> = list_dir(path)
        .into_iter()
        .filter(|item| item.extension().unwrap() == "json")
        .map(|p| p.name_without_extension())
        .collect();
    
    column![
        pick_list(files, state.selected_file.as_ref().map(|p| p.name_without_extension()), Message::FileSelected)
            .placeholder("Select your favorite theme"),
        button(text("Process"))
            .on_press(Message::Execute),
        button(text("Pick file"))
            .on_press(Message::PickFile)
    ].into()
}

pub fn gui() {        
    let init_state = State {
        selected_file: None,
        current_theme: Theme::Dark,
        config: parse_config(),
    };
    
    iced::application(move || init_state.clone(), update, view)
        .theme(|state: &State| state.current_theme.clone())
        .run()
        .unwrap();
}