pub mod album;
pub mod track;

// --- std ---
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
};
// --- external ---
use reqwest::{Client, Response};

pub struct Fetcher(Client);

impl Fetcher {
    fn new(client: Client) -> Fetcher { Fetcher(client) }

    fn get(&self, url: &str) -> Response {
        loop {
            match self.0.get(url).send() {
                Ok(resp) => return resp,
                Err(_) => continue,
            }
        }
    }

    pub fn fetch_to_temp_file(&self, url: &str, path: &Path) -> PathBuf {
        let mut image = vec![];
        self.get(url).copy_to(&mut image).unwrap();

        let path = path.join(url.split("/").last().unwrap());
        let mut file = File::create(&path).unwrap();
        file.write_all(&mut image).unwrap();
        file.sync_all().unwrap();

        path.to_owned()
    }
}

lazy_static! {
    pub static ref FETCHER: Arc<Fetcher> = {
        let mut header = reqwest::header::HeaderMap::new();
        header.insert(reqwest::header::USER_AGENT, "Mozilla/5.0".parse().unwrap());

        Arc::new(Fetcher::new(
            reqwest::ClientBuilder::new()
                .danger_accept_invalid_certs(false)
                .danger_accept_invalid_hostnames(false)
                .default_headers(header)
                .build()
                .unwrap()
        ))
    };
}
