# blackrock

> A simple cli app for downloading YouTube videos.

Based on [bitreel](https://github.com/archer884/bitreel) ("BR"), this downloader ("BRD" -- get it? Ha!) is just a convenient way to grab videos from YouTube.

## Usage

```sh
$ brd <url> [<output path>] [--format <format code>]
$
```

The program doesn't actually produce any output; it just saves the video to `<id>.mp4` in your working directory.

You can also use the program to list available formats for a given video. (Note: YouTube pretty much only makes available three formats).

```sh
$ brd <id> --list-formats
small
medium
hd720
```

If a format is not provided for the download command, it will download the largest available format.

## Changelog

* **v0.2:** Demoted format argument in favor of an optional output file path. This seems like it would be way more useful in most cases.

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

[LICENSE-APACHE]: https://github.com/archer884/bitreel/blob/master/LICENSE-MIT
[LICENSE-MIT]: https://github.com/archer884/bitreel/blob/master/LICENSE-APACHE
