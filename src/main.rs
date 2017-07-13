#[macro_use]
extern crate clap;

extern crate bitreel;
extern crate reqwest;

mod command;
mod connector;
mod error;

use bitreel::client::*;
use bitreel::video::*;
use error::*;

fn main() {
    use command::Command;

    let command = match command::from_args() {
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
        Ok(command) => command,
    };

    let client = YoutubeClient::new(connector::ReqwestConnector);
    let result = match command {
        Command::Download { id, format } => download(id, format, &client),
        Command::ListFormats { id } => list_formats(id, &client),
    };

    if let Err(e) = result {
        println!("{}", e);
        std::process::exit(2);
    }
}

fn download<T1, T2, C>(id: T1, format: Option<T2>, client: &C) -> Result<()>
    where
        T1: AsRef<str>,
        T2: AsRef<str>,
        C: Client,
{
    use std::fs::File;
    use std::io;

    let id = id.as_ref();
    let video = client.query(id).map_err(|e| Error::not_found(e))?;
    let format = match format.as_ref() {
        None => video.list_formats().last().ok_or(Error::format_unavailable())?,
        Some(format) => format.as_ref(),
    };

    let url = video.get_url(format).ok_or(Error::format_unavailable())?;
    let mut file = File::create(&(id.to_owned() + ".mp4"))?;
    let mut res = reqwest::get(url)?;

    io::copy(&mut res, &mut file)?;

    Ok(())
}

fn list_formats<T: AsRef<str>, C: Client>(id: T, client: &C) -> Result<()> {
    let id = id.as_ref();
    let video = client.query(id).map_err(|e| Error::not_found(e))?;

    for format in video.list_formats() {
        println!("{}", format);
    }

    Ok(())
}
