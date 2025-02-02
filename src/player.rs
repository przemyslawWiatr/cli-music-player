use std::{fmt::Display, io::stdout};

use crossterm::{
    event::{read, Event, KeyCode, KeyEventKind},
    style::{style, Color, Stylize},
    terminal, ExecutableCommand,
};
use reqwest::blocking::Client;
use serde_json::{from_str, Value};

pub struct Player {
    client: Client,
    song: Option<Value>,
}

impl Player {
    pub fn init() -> Self {
        Player {
            client: Client::new(),
            song: None,
        }
    }

    pub fn search(&mut self, query: &str) {
        // send request to deezer's api
        let response = self
            .client
            .get(format!("https://api.deezer.com/search?q={query}"))
            .send()
            .unwrap();
        let json = from_str::<Value>(response.text().unwrap().as_str()).unwrap();
        let song_collection = json.get("data").unwrap().as_array().unwrap();

        // handle user interface
        terminal::enable_raw_mode().unwrap();
        let mut stdout = stdout();
        let mut hover_index = 0;
        'song_selection: loop {
            // display graphic interface
            stdout
                .execute(terminal::Clear(terminal::ClearType::All))
                .unwrap();

            println!("arrow up, arrow down - choose song, enter - confirm selection, escape - quit song selection");
            for (position, song) in song_collection.iter().enumerate() {
                let output: Box<dyn Display>;
                if position == hover_index {
                    output = Box::new(
                        style(format!("{}. {}", position, song.get("title").unwrap()))
                            .with(Color::Black)
                            .on(Color::White),
                    );
                } else {
                    output = Box::new(format!("{}. {}", position, song.get("title").unwrap()));
                }
                println!("{output}");
            }

            // handle user input
            match read().unwrap() {
                Event::Key(key_event) => {
                    if let KeyEventKind::Release = key_event.kind {
                        match key_event.code {
                            KeyCode::Up => {
                                if hover_index > 0 {
                                    hover_index -= 1;
                                }
                            }
                            KeyCode::Down => {
                                if hover_index < song_collection.len() {
                                    hover_index += 1;
                                }
                            }
                            KeyCode::Enter => {
                                self.song = Some(song_collection[hover_index].clone());
                                break 'song_selection;
                            }
                            KeyCode::Esc => break 'song_selection,
                            _default => (),
                        }
                    }
                }
                _default => (),
            }
        }
        terminal::disable_raw_mode().unwrap();
    }
}
