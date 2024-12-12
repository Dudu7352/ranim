use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct DisplayArgs {
    pub file: String,

    #[arg(short = 'W', long)]
    pub width: u32
}