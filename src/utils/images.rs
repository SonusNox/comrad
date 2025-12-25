use eframe::CreationContext;

use egui::{
    Image,
    load::Bytes
};

use crate::utils::filesys;

const ADD: &str = "./assets/add.png";
const MINI_PLAYER: &str = "./assets/mini_player.png";
const PAUSE: &str = "./assets/pause.png";
const PLAY: &str = "./assets/play.png";
const PLAYLIST: &str = "./assets/playlist.png";
const PLAYLIST_ADD: &str = "./assets/playlist_add.png";
const REPEAT: &str = "./assets/repeat.png";
const REPEAT_OFF: &str = "./assets/repeat_off.png";
const REPEAT_ONE: &str = "./assets/repeat_one.png";
const SHUFFLE: &str = "./assets/shuffle.png";
const SHUFFLE_OFF: &str = "./assets/shuffle_off.png";
const SKIP_BACKWARD: &str = "./assets/skip_backward.png";
const SKIP_FORWARD: &str = "./assets/skip_forward.png";
const STOP: &str = "./assets/stop.png";
const VOLUME_DOWN: &str = "./assets/volume_down.png";
const VOLUME_MUTE: &str = "./assets/volume_mute.png";
const VOLUME_NONE: &str = "./assets/volume_none.png";
const VOLUME_UP: &str = "./assets/volume_up.png";

pub fn get_add() -> Image<'static> {
    Image::new("bytes://".to_owned() + ADD)
}

pub fn get_mini_player() -> Image<'static> {
    Image::new("bytes://".to_owned() + MINI_PLAYER)
}

pub fn get_pause() -> Image<'static> {
    Image::new("bytes://".to_owned() + PAUSE)
}

pub fn get_play() -> Image<'static> {
    Image::new("bytes://".to_owned() + PLAY)
}

pub fn get_playlist() -> Image<'static> {
    Image::new("bytes://".to_owned() + PLAYLIST)
}

pub fn get_playlist_add() -> Image<'static> {
    Image::new("bytes://".to_owned() + PLAYLIST_ADD)
}

pub fn get_repeat() -> Image<'static> {
    Image::new("bytes://".to_owned() + REPEAT)
}

pub fn get_repeat_off() -> Image<'static> {
    Image::new("bytes://".to_owned() + REPEAT_OFF)
}

pub fn get_repeat_one() -> Image<'static> {
    Image::new("bytes://".to_owned() + REPEAT_ONE)
}

pub fn get_shuffle() -> Image<'static> {
    Image::new("bytes://".to_owned() + SHUFFLE)
}

pub fn get_shuffle_off() -> Image<'static> {
    Image::new("bytes://".to_owned() + SHUFFLE_OFF)
}

pub fn get_skip_backward() -> Image<'static> {
    Image::new("bytes://".to_owned() + SKIP_BACKWARD)
}

pub fn get_skip_forward() -> Image<'static> {
    Image::new("bytes://".to_owned() + SKIP_FORWARD)
}

pub fn get_stop() -> Image<'static> {
    Image::new("bytes://".to_owned() + STOP)
}

pub fn get_volume_down() -> Image<'static> {
    Image::new("bytes://".to_owned() + VOLUME_DOWN)
}

pub fn get_volume_mute() -> Image<'static> {
    Image::new("bytes://".to_owned() + VOLUME_MUTE)
}

pub fn get_volume_none() -> Image<'static> {
    Image::new("bytes://".to_owned() + VOLUME_NONE)
}

pub fn get_volume_up() -> Image<'static> {
    Image::new("bytes://".to_owned() + VOLUME_UP)
}

pub fn load(cc: &CreationContext<'_>) {
    cc.egui_ctx.include_bytes(
        "bytes://".to_owned() + ADD,
        Bytes::from(filesys::get_file(ADD.to_string()).unwrap())
    );

    cc.egui_ctx.include_bytes(
        "bytes://".to_owned() + MINI_PLAYER,
        Bytes::from(filesys::get_file(MINI_PLAYER.to_string()).unwrap())
    );

    cc.egui_ctx.include_bytes(
        "bytes://".to_owned() + PAUSE,
        Bytes::from(filesys::get_file(PAUSE.to_string()).unwrap())
    );

    cc.egui_ctx.include_bytes(
        "bytes://".to_owned() + PLAY,
        Bytes::from(filesys::get_file(PLAY.to_string()).unwrap())
    );

    cc.egui_ctx.include_bytes(
        "bytes://".to_owned() + PLAYLIST,
        Bytes::from(filesys::get_file(PLAYLIST.to_string()).unwrap())
    );

    cc.egui_ctx.include_bytes(
        "bytes://".to_owned() + PLAYLIST_ADD,
        Bytes::from(filesys::get_file(PLAYLIST_ADD.to_string()).unwrap())
    );

    cc.egui_ctx.include_bytes(
        "bytes://".to_owned() + REPEAT,
        Bytes::from(filesys::get_file(REPEAT.to_string()).unwrap())
    );

    cc.egui_ctx.include_bytes(
        "bytes://".to_owned() + REPEAT_OFF,
        Bytes::from(filesys::get_file(REPEAT_OFF.to_string()).unwrap())
    );

    cc.egui_ctx.include_bytes(
        "bytes://".to_owned() + REPEAT_ONE,
        Bytes::from(filesys::get_file(REPEAT_ONE.to_string()).unwrap())
    );

    cc.egui_ctx.include_bytes(
        "bytes://".to_owned() + SHUFFLE,
        Bytes::from(filesys::get_file(SHUFFLE.to_string()).unwrap())
    );

    cc.egui_ctx.include_bytes(
        "bytes://".to_owned() + SHUFFLE_OFF,
        Bytes::from(filesys::get_file(SHUFFLE_OFF.to_string()).unwrap())
    );

    cc.egui_ctx.include_bytes(
        "bytes://".to_owned() + SKIP_BACKWARD,
        Bytes::from(filesys::get_file(SKIP_BACKWARD.to_string()).unwrap())
    );

    cc.egui_ctx.include_bytes(
        "bytes://".to_owned() + SKIP_FORWARD,
        Bytes::from(filesys::get_file(SKIP_FORWARD.to_string()).unwrap())
    );

    cc.egui_ctx.include_bytes(
        "bytes://".to_owned() + STOP,
        Bytes::from(filesys::get_file(STOP.to_string()).unwrap())
    );

    cc.egui_ctx.include_bytes(
        "bytes://".to_owned() + VOLUME_DOWN,
        Bytes::from(filesys::get_file(VOLUME_DOWN.to_string()).unwrap())
    );

    cc.egui_ctx.include_bytes(
        "bytes://".to_owned() + VOLUME_MUTE,
        Bytes::from(filesys::get_file(VOLUME_MUTE.to_string()).unwrap())
    );

    cc.egui_ctx.include_bytes(
        "bytes://".to_owned() + VOLUME_NONE,
        Bytes::from(filesys::get_file(VOLUME_NONE.to_string()).unwrap())
    );

    cc.egui_ctx.include_bytes(
        "bytes://".to_owned() + VOLUME_UP,
        Bytes::from(filesys::get_file(VOLUME_UP.to_string()).unwrap())
    );
}