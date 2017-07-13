#[macro_use]
extern crate clap;

extern crate bitreel;
extern crate reqwest;

mod command;
mod connector;
mod error;
mod format;

use bitreel::client::*;
use bitreel::video::*;
use error::*;
use format::Format;

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

fn download<S: AsRef<str>, C: Client>(id: S, format: Option<Format>, client: &C) -> Result<()> {
    use std::fs::File;
    use std::io::{self, BufReader, BufWriter};

    let id = id.as_ref();
    let video = client.query(id).map_err(|e| Error::not_found(e))?;
    let format = format
        .or_else(|| {
            video.list_formats()
                .filter_map(|format| format.parse::<Format>().ok())
                .max()
        })
        .ok_or_else(|| Error::format_unavailable())?;

    let url = video.get_url(format.as_ref()).ok_or(Error::format_unavailable())?;
    let mut file = BufWriter::new(File::create(&(id.to_owned() + ".mp4"))?);
    let mut res = BufReader::new(reqwest::get(url)?);

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
