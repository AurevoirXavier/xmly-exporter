extern crate clipboard;
#[macro_use]
extern crate conrod;
extern crate image;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate reqwest;
extern crate serde_json;
extern crate tempfile;

mod fetcher;
mod ui;

fn main() { ui::display(); }
