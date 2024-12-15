use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct DisplayArgs {
    pub file: String,

    #[arg(short = 'W', long)]
    pub width: Option<u32>,
}

pub enum DisplaySize {
    Width(u32),
    Fill
}