use bitreel::video::StreamKey;
use error::*;

/// Attempts to create a command based on program arguments.
pub fn from_args() -> Result<Command> {
    let app_matches = clap_app!(blackrock => 
        (author: crate_authors!())
        (version: crate_version!())
        (about: "YouTube video downloader based on bitreel.")
        (@arg video: +required "The ID of the video to be downloaded")
        (@arg path: "The path at which to store downloaded video")
        (@arg format: -f --format "The format of the video to be downloaded")
        (@arg list: -l --list-formats "List formats instead of downloading the video")
    ).get_matches();

    if app_matches.is_present("list") {
        Ok(Command::ListFormats { id: app_matches.value_of("video").unwrap().into() })
    } else {
        let format = match app_matches.value_of("format") {
            None => None,
            Some(format) => Some(format.parse::<StreamKey>().map_err(|_| Error::format_unsupported())?),
        };

        let output_path = app_matches.value_of("path").map(|s| s.to_owned());

        Ok(Command::Download {
            id: app_matches.value_of("video").unwrap().into(),
            format,
            output_path,
        })
    }
}

pub enum Command {
    /// Represents a request to download a video.
    ///
    /// Optionally, a download command may include a specific format to be downloaded. If this 
    /// is not included, blackrock will attempt to download the highest definition available.
    Download { id: String, format: Option<StreamKey>, output_path: Option<String> },

    /// Represents a request to list available formats for a video.
    ListFormats { id: String }
}
