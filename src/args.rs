use clap::Parser;

/// A Simple CLI tool to translate subtitle files
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to the subtitle file
    #[arg(short, long)]
    pub path: String,

    /// The language of the subtitle file
    #[arg(short, long)]
    #[clap(visible_alias=&"sl")]
    pub source_language: String,

    /// The language to translate the subtitle file to
    #[arg(short, long)]
    #[clap(visible_alias=&"tl")]
    pub target_language: String,
}
