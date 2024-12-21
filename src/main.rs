use std::{fs::File, io::BufReader, process::exit};

use args::{DisplayArgs, DisplaySize};
use clap::Parser;
use display::{clean, display_anim};
use image::{codecs::gif::GifDecoder, AnimationDecoder};
use render::render_frame;
use types::StrFrame;

mod args;
mod display;
mod render;
mod terminal_consts;
mod types;
fn main() {
    let args = DisplayArgs::parse();
    let desired_size = if let Some(desired_width) = args.width {
        DisplaySize::Width(desired_width)
    } else {
        DisplaySize::Fill
    };

    let _ = ctrlc::set_handler(|| {
        clean();
        exit(0);
    });

    let file_in = BufReader::new(File::open(&args.file).unwrap());
    let decoder = GifDecoder::new(file_in).unwrap();
    let frames = decoder.into_frames();
    let generated: Vec<StrFrame> = frames
        .map(|f| render_frame(f.unwrap(), &desired_size))
        .collect();
    display_anim(generated, &args);
}
