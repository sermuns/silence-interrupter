use clap::Parser;
use rust_embed::Embed;
use std::{
    io::{Cursor, Read, Seek},
    time::{Duration, Instant},
};

mod cli;

use crate::cli::Args;

#[derive(Embed)]
#[folder = "audio"]
struct Audio;

fn main() -> simple_eyre::Result<()> {
    simple_eyre::install()?;

    let args = Args::parse();
    dbg!(&args);

    let audio_device = rodio::DeviceSinkBuilder::open_default_sink()?;
    let mut audio_player = rodio::Player::connect_new(audio_device.mixer());
    let num_audio_files = Audio::iter().count();

    let mut next_interruption = random_future_instant();

    loop {
        if next_interruption.elapsed() > Duration::ZERO {
            println!("happened!");
            audio_player = rodio::play(audio_device.mixer(), get_random_audio(num_audio_files))?;
            next_interruption = random_future_instant();
        }
    }
}

fn random_future_instant() -> Instant {
    Instant::now() + Duration::from_secs(fastrand::u64(1..5))
}

fn get_random_audio(num_audio_files: usize) -> impl Read + Seek + Sync + 'static {
    let random_audio_index = fastrand::usize(..num_audio_files);
    let audio_path = Audio::iter().nth(random_audio_index).unwrap();

    let audio_data = Audio::get(&audio_path).unwrap().data;
    Cursor::new(audio_data)
}
