use std::{ops::Range, time::Duration};
use simple_eyre::eyre::OptionExt;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Possible random time range
    #[arg(
        short,
        long, 
        value_parser = parse_duration_range,
        value_name = "start>..<end",
    )]
    pub range: Range<Duration>,

    // Audio is multiplied by this to lower audio.
    // Probably a bad idea to have this over 1.0 ...
    #[arg(short, long, default_value_t = 1.0)]
    pub gain: f32,
}

fn parse_duration_range(s: &str) -> simple_eyre::Result<Range<Duration>> {
    let (start_str, end_str) = s.split_once("..").ok_or_eyre("Invalid range format. Expected format: <start>..<end>")?;
    let start = humantime::parse_duration(start_str)?;
    let end = humantime::parse_duration(end_str)?;
    Ok(start..end)
}
