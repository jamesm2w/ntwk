use std::env;
use iced::pure::Sandbox;

use iced::Settings;
use iced::window;
use iced::window::Position;

pub mod netwk;
pub mod canvas;
pub mod ui;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|x| x.as_str()).unwrap_or("ui") {
        "ui" => {

            let default_settings = Settings::default();
            let default_window = window::Settings::default();

            ui::NetworkUI::run(Settings {
                window: window::Settings {
                    size: (500, 600),
                    position: Position::Centered,
                    ..default_window
                },
                ..default_settings
            }).expect("Application exited with error")
        }
        _ => {
            println!("Hello World!");
        }
    }
}
