## Intro

#### Installation

1.  Complie from source (recommend)
   - `git clone https://github.com/AurevoirXavier/xmly-exporter.git`
   - `cargo build --release` (rust version 1.33.0 nightly)
2.  Download relase
   - [None](#)

#### Usage

1. Copy url (https://www.ximalaya.com/toutiao/4308484/, https://www.ximalaya.com/toutiao/4308484/147135825). Album and Track are supported
2. **Fetch**: Click it and it will read the url from your clipboard to start fetching
3. **Export All**: Export all tracks’ detail to a \*.ax file which for [aria2](https://aria2.github.io)’s -i flag. Cause some problem with *Async*, download are not supported now.
4. Button: **Color** change when click
   - **Click to copy download link**: Just as it told (also copy the title of the track)
   - **Track id**, **Album**, **Album id**: Click to copy
5. List select: Click to get the track’s detail as below 
6. Text: Category, Nickname, Duration, Plays, Comments, Shares, Likes

