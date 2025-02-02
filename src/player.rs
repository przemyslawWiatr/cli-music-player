use reqwest::blocking::Client;
use serde_json::{from_str, Value};

pub struct Player {
    client: Client,
}

impl Player {
    pub fn init() -> Self {
        Player {
            client: Client::new(),
        }
    }

    pub fn search(&self, query: &str) {
        let response = self
            .client
            .get(format!("https://api.deezer.com/search?q={query}"))
            .send()
            .unwrap();
        let json = from_str::<Value>(response.text().unwrap().as_str()).unwrap();
        let song_collection = json.get("data").unwrap().as_array().unwrap();

        for (position, song) in song_collection.iter().enumerate() {
            println!("{}. {}", position, song.get("title").unwrap());
        }
    }
}
