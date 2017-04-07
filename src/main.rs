extern crate bitreel;
extern crate hyper;
extern crate hyper_native_tls;

use bitreel::client::*;
use bitreel::video::*;
use std::fs::File;

fn main() {
    let client = get_client();
    let youtube_client = YoutubeClient::new();

    if let Some((id, format)) = get_query() {
        match youtube_client.query(&id, &client) {
            Err(e) => {
                println!("{}", e);
                std::process::exit(1);
            },

            Ok(video) => {
                use std::io;

                let url = video.format(&format).expect("format unavailable");
                let mut file = File::create(&id).expect("can't save file");
                let mut res = client.get(url).send().expect("resource not found");

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

fn get_client() -> hyper::Client {
    use hyper::net::HttpsConnector;
    use hyper_native_tls::NativeTlsClient;

    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    
    hyper::Client::with_connector(connector)
}
