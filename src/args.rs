use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Utility that displays GIFs in terminal", long_about = None)]
pub struct DisplayArgs {
    /// GIF file to display
    pub file: String,

    /// Width to display. Without this flag GIF will be stretched to fill the screen
    #[arg(short = 'W', long)]
    pub width: Option<u32>,

    /// Fit animation to the terminal screen
    #[arg(long, conflicts_with = "width")]
    pub fit: bool,

    /// Loops animation infinitely, press Ctrl-C to escape
    #[arg(long = "loop")]
    pub loop_animation: bool,

    /// Centers animation on terminal screen
    #[arg(short, long)]
    pub center: bool,
}

pub enum DisplaySize {
    Width(u32),
    Fill,
    Fit
}
