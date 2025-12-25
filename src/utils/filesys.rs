use std::{
    fs::{
        self,
        OpenOptions,
        ReadDir,
        read,
        read_dir as read_from_dir
    },
    io::{Result, Write},
    path::Path
};

use crate::media::source::Playlist;

fn dir() -> String { "./bin".to_string() }
fn config_file() -> String { "config.ini".to_string() }
fn config_path() -> String { (dir() + "/" + &config_file()).to_string() }
fn playlists_file() -> String { "playlists.ini".to_string() }
fn playlists_path() -> String { (dir() + "/" + &playlists_file()).to_string() }

fn create_dir(path: String) -> Result<()> {
    if !exists(path.clone()) { fs::create_dir(path)?; };

    Ok(())
}

pub fn create_config() -> Result<()> {
    let _ = create_dir(dir());

    if exists(dir()) {
        if !exists(config_path()) {
            let mut f = OpenOptions::new().create_new(true).write(true).truncate(true)
                .open(config_path()).unwrap();

            let _ = writeln!(&mut f, "/").map_err(|error| println!("{:?}", error));
            let _ = writeln!(&mut f, "100").map_err(|error| println!("{:?}", error));
        };
    };

    Ok(())
}

pub fn create_from_path(path: String) -> String {
    let name = Path::new(&path).file_name();
    
    if name.is_some() { name.unwrap().display().to_string() }
    else { String::new() }
}

pub fn create_playlists() -> Result<()> {
    if exists(dir()) {
        if !exists(playlists_path()) {
            let mut f = OpenOptions::new().create_new(true).write(true).open(playlists_path())
                .unwrap();

            let _ = writeln!(&mut f, "").map_err(|error| println!("{:?}", error));
        };
    };

    Ok(())
}

pub fn edit_config(directory: String, volume: i32) -> Result<()> {
    if exists(config_path()) {
        let mut f = OpenOptions::new().write(true).open(config_path()).unwrap();

        writeln!(&mut f, "{}", directory)?;
        writeln!(&mut f, "{}", volume)?;
    };

    Ok(())
}

pub fn edit_playlists(playlists: Vec<Playlist>) -> Result<()> {
    if exists(playlists_path()) {
        let mut f = OpenOptions::new().write(true).truncate(true).open(playlists_path()).unwrap();

        for playlist in playlists {
            let mut line = String::new();

            let id = playlist.get_id().unwrap();
            let name = playlist.get_name().unwrap();
            let sources = playlist.get_sources();

            if let Some(list) = &sources {
                for source in list { line = line + "⁙" + &source.replace("\\", "/"); };
            };

            writeln!(&mut f, "{:?}⁘{:?}⁘{:?}", id, name, line)?;
        };
    };

    Ok(())
}

pub fn exists(dir: String) -> bool { Path::new(&dir).exists() }

pub fn get_dir() -> String {
    read_file(config_path()).split("\n").collect::<Vec<_>>().get(0).unwrap().to_string()
}

pub fn get_file(file: String) -> Result<Vec<u8>> {
    read(file)
}

pub fn get_playlists() -> Vec<Playlist> {
    let file = read_file(playlists_path());
    let lines = file.split("\n").collect::<Vec<_>>();
    let mut playlists: Vec<Playlist> = Vec::new();

    for line in lines {
        if *line != String::new() {
            let items = line.split("⁘").collect::<Vec<_>>();
            let source_line = items.get(2).unwrap();
            let sources = source_line.split("⁙").collect::<Vec<_>>();

            let id = items.get(0).unwrap().to_string().replace(r#"""#, "").replace(r#"\"#, "");
            let name = items.get(1).unwrap().to_string().replace(r#"""#, "").replace(r#"\"#, "");
            let mut playlist = Playlist::new(name.clone());
            
            playlist.set_id(id.clone());

            for source in &sources.clone() {
                if source != sources.get(0).unwrap() {
                    let source = source.to_string().replace(r#"""#, "").replace(r#"\"#, "");

                    playlist.add_source(Some(source.clone()));
                };
            };

            playlists.push(playlist);
        };
    };

    playlists
}

pub fn get_volume() -> i32 {
    if !exists(config_path()) { return 100; }

    read_file(config_path()).split("\n").collect::<Vec<_>>().get(1).unwrap().parse::<i32>().unwrap()
}

pub fn read_dir(dir: String) -> Result<ReadDir> {
    Ok(read_from_dir(dir)?)
}

fn read_file(path: String) -> String {
    if !exists(path.clone()) { return "/".to_string(); }

    fs::read_to_string(path).expect("Unable to read contents of file.").to_string()
}