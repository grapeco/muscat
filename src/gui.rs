use std::{fs, path::{Path, PathBuf}};

use hex_color::HexColor;
use iced::{Color, Element, Theme, widget::{button, column, pick_list, text}};
use resolve_path::PathResolveExt;
use serde::Deserialize;

use super::func::func::{self, execute, parse_config, list_dir};

#[derive(Default, Clone, Deserialize)]
struct Base16 {
    base00: String,
    base05: String,
    base08: String,
    base0A: String,
    base0B: String,
    base0D: String,
}

#[derive(Clone)]
struct State {
    selected_file: Option<String>,
    current_theme: Theme,
}

#[derive(Clone)]
enum Message {
    FileSelected(String),
    Execute,
}

fn hex_to_color(hex: String) -> Color {
    let parsed = match hex.starts_with('#') {
        true => HexColor::parse_rgb(&hex[1..]).unwrap(),
        false => HexColor::parse_rgb(format!("#{}", hex).as_str()).unwrap()
    };
    
    return Color::from_rgb(
        parsed.r as f32 / 255.0, 
        parsed.g as f32 / 255.0, 
        parsed.b as f32 / 255.0,
    );
}

fn load_theme_from_file(filename: &str) -> Theme {
    let path = format!("~/.config/muscat/themes/{}", filename);
    let content = fs::read_to_string(path.resolve()).unwrap();
    let palette: Base16 = serde_json::from_str(&content).unwrap();
    
    let theme_name = filename.trim_end_matches(".json").to_string();
    
    return Theme::custom(
        theme_name,
        iced::theme::Palette {
            background: hex_to_color(palette.base00),
            text: hex_to_color(palette.base05),
            primary: hex_to_color(palette.base0D),
            success: hex_to_color(palette.base0B),
            warning: hex_to_color(palette.base0A),
            danger: hex_to_color(palette.base08),
        }
    );
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::FileSelected(file) => {
            state.selected_file = Some(file.clone());
            state.current_theme = load_theme_from_file(&file);
            
            // let path = rfd::AsyncFileDialog::new()
            //         .pick_file()
            //         .await;
            // println!("Path: {:?}", path);
        }
        Message::Execute => {
            let targets = parse_config().targets
                .iter()
                .map(|target| target.resolve().to_path_buf())
                .collect();
            
            execute(
                targets,
                PathBuf::from(format!(
                    "{}/{}",
                    "~/.config/muscat/themes".resolve().display(),
                    state.selected_file.as_ref().unwrap()
                ))
            );
            
            func::restart();
        }
    }
}

fn view(state: &State) -> Element<'_, Message> {
    let files: Vec<String> = list_dir("~/.config/muscat/themes".resolve())
        .into_iter()
        .filter(|item| Path::new(item).extension().unwrap() == "json")
        .collect();
    
    column![
        pick_list(files, state.selected_file.as_ref(), Message::FileSelected)
            .placeholder("Select your favorite theme"),
        button(text("Process"))
            .on_press(Message::Execute),
    ].into()
}

pub fn gui() {    
    let init_state = State {
        selected_file: None,
        current_theme: Theme::Dark,
    };
    
    iced::application(move || init_state.clone(), update, view)
        .theme(|state: &State| state.current_theme.clone())
        .run()
        .unwrap();
}