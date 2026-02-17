use std::path::{Path, PathBuf};

use iced::{Element, widget::{button, column, pick_list, text}};
use resolve_path::PathResolveExt;

use crate::func::{self, execute, list_dir, parse_config};

#[derive(Default)]
struct State {
    selected_file: Option<String>,
}

#[derive(Clone)]
enum Message {
    FileSelected(String),
    Execute,
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::FileSelected(file) => state.selected_file = Some(file),
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
            
            tokio::spawn(async move {
                func::restart().await;
            });
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
            .placeholder("Select your favorite fruit..."),
        button(text("Process"))
            .on_press(Message::Execute)
    ].into()
}

pub fn gui() {
    iced::application(State::default, update, view)
        .theme(iced::Theme::CatppuccinMocha)
        .run()
        .unwrap();
}