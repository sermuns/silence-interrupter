use clap::Parser;
use rust_embed::Embed;
use std::{
    io::{Cursor, Read, Seek},
    ops::Range,
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

    let audio_device = rodio::DeviceSinkBuilder::open_default_sink()?;
    let audio_player = rodio::Player::connect_new(audio_device.mixer());
    audio_player.set_volume(args.gain);

    let num_audio_files = Audio::iter().count();

    let mut next_interruption = random_future_instant(&args.range);

    loop {
        if next_interruption.elapsed() > Duration::ZERO {
            audio_player.clear();
            audio_player.append(rodio::Decoder::new_vorbis(get_random_audio(
                num_audio_files,
            ))?);
            audio_player.play();

            next_interruption = random_future_instant(&args.range);
        }
    }
}

fn random_future_instant(range: &Range<Duration>) -> Instant {
    let t = fastrand::f64();
    let seconds = range.start.as_secs_f64() * (1. - t) + range.end.as_secs_f64() * t;
    let duration = Duration::from_secs_f64(seconds);
    Instant::now() + duration
}

fn get_random_audio(num_audio_files: usize) -> impl Read + Seek + 'static {
    let random_audio_index = fastrand::usize(..num_audio_files);
    let audio_path = Audio::iter().nth(random_audio_index).unwrap();
    println!("playing '{}'", audio_path);

    let audio_data = Audio::get(&audio_path).unwrap().data;
    Cursor::new(audio_data)
}
