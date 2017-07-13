use bitreel::client::ClientConnector;
use reqwest;

pub struct ReqwestConnector;

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
