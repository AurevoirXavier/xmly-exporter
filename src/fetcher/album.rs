// --- custom ---
use super::{
    FETCHER,
    track::Track,
};

const ALBUM_URL: &'static str = "https://www.ximalaya.com/revision/play/album?albumId=";

#[derive(Debug)]
pub struct Album {
    url: String,
    name: String,
    pub tracks: Vec<Track>,
}

impl Album {
    pub fn new() -> Album {
        Album {
            url: String::new(),
            name: String::new(),
            tracks: vec![],
        }
    }

    pub fn set_id(&mut self, id: &str) -> &mut Self {
        self.url = format!("{}{}", ALBUM_URL, id);
        self
    }

    fn get_album_name(&mut self) -> String {
        let resp: serde_json::Value = FETCHER.get(&format!("{}&pageNum=1&pageSize=1", self.url))
            .json()
            .unwrap();

        resp["data"]["tracksAudioPlay"]
            .as_array()
            .unwrap()[0]["albumName"]
            .as_str()
            .unwrap()
            .to_owned()
    }

    pub fn next_page(&mut self, page_num: u32) -> bool {
        let fetcher = FETCHER.clone();

        if let Ok(resp) = fetcher.get(&format!("{}&pageNum={}", self.url, page_num)).json::<serde_json::Value>() {
            let data = &resp["data"];

            {
                let tracks = data["tracksAudioPlay"].as_array().unwrap();
                if tracks.is_empty() { return false; }
                for track in tracks { self.tracks.push(Track::from_json(track)); }
            }

            if !data["hasMore"].as_bool().unwrap() {
                self.name = self.get_album_name();
                false
            } else { true }
        } else { false }
    }

    pub fn save_aria2_input_file(&self) -> &Self {
        // --- std ---
        use std::fs::write;

        let dir = &self.name;
        let mut tracks = String::new();

        for track in self.tracks.iter() {
            let src = &track.src;

            tracks.push_str(&format!(
                "{}\n\tout={}.{}\n\tdir={}\n",
                src,
                track.title,
                src.split('.').last().unwrap(),
                dir,
            ));
        }

        write(&format!("{}.ax", dir), tracks).unwrap();

        self
    }
}
