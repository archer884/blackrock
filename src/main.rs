extern crate bitreel;
extern crate reqwest;

use bitreel::client::*;
use bitreel::video::*;
use std::fs::File;

fn main() {
    let youtube_client = YoutubeClient::new();

    if let Some((id, format)) = get_query() {
        match youtube_client.query(&id, &ReqwestConnector) {
            Err(e) => {
                println!("{}", e);
                std::process::exit(1);
            },

            Ok(video) => {
                use std::io;

                let url = video.format(&format).expect("format unavailable");
                let mut file = File::create(&id).expect("can't save file");
                let mut res = reqwest::get(url).expect("resource not found");

                io::copy(&mut res, &mut file).expect("unable to write to file");
            }
        }
    }
}

fn get_query() -> Option<(String, String)> {
    let mut args = std::env::args().skip(1).take(2);
    let left = args.next();
    let right = args.next();

    left.and_then(|x| right.map(|y| (x, y)))
}

struct ReqwestConnector;

impl ClientConnector for ReqwestConnector {
    type Err = reqwest::Error;
    fn download_string(&self, uri: &str) -> Result<String, Self::Err> {
        use std::io::Read;
        let mut response = reqwest::get(uri)?;
        let mut buf = String::new();
        response.read_to_string(&mut buf).unwrap();
        Ok(buf)
    }
}
