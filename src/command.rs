use error::*;

/// Attempts to create a command based on program arguments.
pub fn from_args() -> Result<Command> {
    let app_matches = clap_app!(blackrock => 
        (author: crate_authors!())
        (version: crate_version!())
        (about: "YouTube video downloader based on bitreel.")
        (@arg video: +required +takes_value "The ID of the video to be downloaded")
        (@arg format: -f --format +takes_value "The format of the video to be downloaded")
        (@arg list: -l --list-formats "List formats instead of downloading the video")
    ).get_matches();

    // Clap should guarantee that this is non-null, so it is not currently possible for this 
    // function to fail. However, I'm still returning a result value in case we introduce any 
    // uncertainty in the future.
    if app_matches.is_present("list") {
        Ok(Command::ListFormats { id: app_matches.value_of("video").unwrap().into() })
    } else {
        Ok(Command::Download {
            id: app_matches.value_of("video").unwrap().into(),
            format: app_matches.value_of("format").map(|s| s.into())
        })
    }
}

pub enum Command {
    /// Represents a request to download a video.
    ///
    /// Optionally, a download command may include a specific format to be downloaded. If this 
    /// is not included, blackrock will attempt to download the highest definition available,
    /// based on the assumption that formats are listed in order of ascending definition.
    Download { id: String, format: Option<String> },

    /// Represents a request to list available formats for a video.
    ListFormats { id: String }
}
