#[macro_use]
extern crate clap;

extern crate bitreel;
extern crate reqwest;

mod command;
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

    let client = Client::new();
    let result = match command {
        Command::Download { id, format, output_path } => download(id, format, output_path, &client),
        Command::ListFormats { id } => list_formats(id, &client),
    };

    if let Err(e) = result {
        println!("{}", e);
        std::process::exit(2);
    }
}

fn download<S: AsRef<str>>(url: S, format: Option<StreamKey>, output_path: Option<String>, client: &Client) -> Result<()> {
    use std::fs::File;
    use std::io::{self, BufReader, BufWriter};

    let url = url.as_ref();
    let video = client.query(url).map_err(|e| Error::not_found(e))?;
    let format = format
        .or_else(|| video.streams().map(|(&key, _)| key).max())
        .ok_or_else(|| Error::format_unavailable())?;

    let url = video.get_stream(format).ok_or(Error::format_unavailable())?;
    let output_path = output_path.unwrap_or_else(|| video.identifier().to_owned());

    let mut file = BufWriter::new(File::create(&output_path)?);
    let mut res = BufReader::new(reqwest::get(url)?);

    io::copy(&mut res, &mut file)?;

    Ok(())
}

// This function cannot be supported given the present state of the bitreel lib. I will need to 
// add a formatter function to the query type enum to allow the stream key to be displayed 
// appropriately. Luckily, it is not strictly necessary to be able to do this at the moment, 
// because we can now parse a stream key from fairly ordinary user input, like small, medium, 
// and large.
fn list_formats<T: AsRef<str>>(_id: T, _client: &Client) -> Result<()> {
    // let id = id.as_ref();
    // let video = client.query(id).map_err(|e| Error::not_found(e))?;

    // for (&format, _) in video.streams() {
    //     println!("{}", format);
    // }

    // Ok(())

    unimplemented!("Sorry, buddy. See comments.")
}
