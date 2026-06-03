use std::{ops::Range, time::Duration};
use humantime::parse_duration;
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
}

fn parse_duration_range(s: &str) -> simple_eyre::Result<Range<Duration>> {
    let (start_str, end_str) = s.split_once("..").ok_or_eyre("Invalid range format. Expected format: <start>..<end>")?;
    let start = parse_duration(start_str)?;
    let end = parse_duration(end_str)?;
    Ok(start..end)
}
