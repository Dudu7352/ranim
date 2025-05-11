use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Utility that displays GIFs in terminal", long_about = None)]
pub struct DisplayArgs {
    /// GIF file to display
    pub file: String,

    /// Width to display.
    #[arg(short = 'W', long)]
    pub width: Option<u32>,

    /// Height to display.
    #[arg(short = 'H', long)]
    pub height: Option<u32>,

    /// Text to display next to the gif.
    #[arg(short, long)]
    pub text: Option<String>,

    /// Fit animation to the terminal screen
    #[arg(long, conflicts_with = "width", conflicts_with = "height")]
    pub fit: bool,

    /// Loops animation infinitely, press Ctrl-C to escape
    #[arg(long = "loop")]
    pub loop_animation: bool,

    /// Centers animation on terminal screen
    #[arg(short, long)]
    pub center: bool,
}

pub enum DisplaySize {
    Size(Option<u32>, Option<u32>),
    Fill,
    Fit,
}
