use rust_embed::Embed;
use std::{
    io::{Cursor, Read, Seek},
    sync::LazyLock,
};

#[derive(Embed)]
#[folder = "audio"]
struct Audio;

static NUM_AUDIO_FILES: LazyLock<usize> = LazyLock::new(|| Audio::iter().count());

pub fn get_random_audio() -> impl Read + Seek + Sync + 'static {
    let random_audio_index = fastrand::usize(..*NUM_AUDIO_FILES);
    let audio_path = Audio::iter().nth(random_audio_index).unwrap();

    let audio_data = Audio::get(&audio_path).unwrap().data;
    Cursor::new(audio_data)
}
