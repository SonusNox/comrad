use audiotags::Tag;

use rodio::{
    Decoder,
    OutputStream,
    OutputStreamBuilder,
    Sink,
    source::Source
};

use std::{
    fs::File,
    io::BufReader,
    path::Path,
    time::Duration
};

use crate::utils::filesys;

pub fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs() % 60;
    let minutes = (duration.as_secs() / 60) % 60;
    let hours = (duration.as_secs() / 60) / 60;

    if hours >= 1 { format!("{hours:0>2}:{minutes:0>2}:{seconds:0>2}") }
    else { format!("{minutes:0>2}:{seconds:0>2}") }
}

pub fn get_album(file_path: Option<String>) -> String {
    let (album_title, _artist, _title) = get_tags(file_path);

    album_title
}

pub fn get_artist(file_path: Option<String>) -> String {
    let (_album_title, artist, _title) = get_tags(file_path);

    artist
}

pub fn get_stream() -> (Sink, OutputStream) {
    let stream = OutputStreamBuilder::open_default_stream().expect("Unable to open stream.");
    let sink = Sink::connect_new(&stream.mixer());

    (sink, stream)
}

fn get_tags(file_path: Option<String>) -> (String, String, String) {
    let mut album_title: String = "".to_string();
    let mut artist: String = "".to_string();
    let mut title: String = "".to_string();

    if file_path != Some("".to_string()) {
        let file: String = file_path.clone().unwrap();
        let tags = Tag::new().read_from_path(&file).unwrap();

        if tags.album_title().is_some() { album_title = tags.album_title().unwrap().to_string(); };
        if tags.artist().is_some() { artist = tags.artist().unwrap().to_string(); };
        if tags.title().is_some() { title = tags.title().unwrap().to_string(); };
    };

    (album_title, artist, title)
}

pub fn get_title(file_path: Option<String>) -> String {
    let (_album_title, _artist, title) = get_tags(file_path.clone());

    title
}

pub fn get_total_time(file_path: &str) -> Duration {
    let mut duration: Duration = {
        if filesys::exists(file_path.to_string()) {
            let file = BufReader::new(File::open(&file_path).unwrap());

            match Path::new(&file_path).extension().and_then(|ext| ext.to_str()) {
                Some(ext) => match ext.to_lowercase().as_str() {
                    "mp3" => mp3_duration::from_path(file_path).unwrap_or(Duration::ZERO),

                    _ => {
                        let source = Decoder::new(file).unwrap();

                        Source::total_duration(&source).unwrap_or(Duration::ZERO)
                    }
                },

                None => Duration::ZERO
            }
        } else { Duration::ZERO }
    };

    if duration != Duration::ZERO { duration += Duration::from_secs(1); };

    duration
}