// --- custom ---
use super::FETCHER;

#[derive(Debug)]
pub struct Track {
    pub id: u64,
    pub album_id: u64,
    pub duration: u64,
    pub plays: u64,
    pub comments: u64,
    pub shares: u64,
    pub likes: u64,
    pub title: String,
    pub album_title: String,
    pub nickname: String,
    pub category: String,
    pub cover: String,
    pub src: String,
}

impl Track {
    pub fn from_json(json: &serde_json::Value) -> Track {
        Track {
            id: json["trackId"].as_u64().unwrap(),
            album_id: 0,
            duration: json["duration"].as_u64().unwrap(),
            plays: 0,
            comments: 0,
            shares: 0,
            likes: 0,
            title: json["trackName"].as_str().unwrap().to_owned(),
            album_title: String::new(),
            nickname: String::new(),
            category: String::new(),
            cover: format!("http://{}", json["trackCoverPath"].as_str().unwrap()),
            src: json["src"].as_str().unwrap().to_owned(),
        }
    }

    pub fn fetch(id: &str) -> Track {
        let json: serde_json::Value = FETCHER.get(&format!("http://www.ximalaya.com/tracks/{}.json", id))
            .json()
            .unwrap();

        Track {
            id: id.parse().unwrap(),
            album_id: json["album_id"].as_u64().unwrap(),
            duration: json["duration"].as_u64().unwrap(),
            plays: json["play_count"].as_u64().unwrap(),
            comments: json["comments_count"].as_u64().unwrap(),
            shares: json["shares_count"].as_u64().unwrap(),
            likes: json["favorites_count"].as_u64().unwrap(),
            title: json["title"].as_str().unwrap().to_owned(),
            album_title: json["album_title"].as_str().unwrap().to_owned(),
            nickname: json["nickname"].as_str().unwrap().to_owned(),
            category: json["category_name"].as_str().unwrap().to_owned(),
            cover: json["cover_url"].as_str().unwrap().to_owned(),
            src: json["play_path"].as_str().unwrap().to_owned(),
        }
    }

    pub fn update(&mut self) {
        if self.album_id != 0 { return; }

        let json: serde_json::Value = FETCHER.get(&format!("http://www.ximalaya.com/tracks/{}.json", self.id))
            .json()
            .unwrap();

        self.album_id = json["album_id"].as_u64().unwrap();
        self.plays = json["play_count"].as_u64().unwrap();
        self.comments = json["comments_count"].as_u64().unwrap();
        self.shares = json["shares_count"].as_u64().unwrap();
        self.likes = json["favorites_count"].as_u64().unwrap();
        self.album_title = json["album_title"].as_str().unwrap().to_owned();
        self.nickname = json["nickname"].as_str().unwrap().to_owned();
        self.category = json["category_name"].as_str().unwrap().to_owned();
    }
}
