use std::{fs::File, io::BufReader, process::exit};

use animation::Animation;
use args::{DisplayArgs, DisplaySize};
use clap::Parser;
use image::{codecs::gif::GifDecoder, AnimationDecoder};
use render::render_frame;
use types::StrFrame;

mod animation;
mod args;
mod render;
mod terminal_consts;
mod types;

fn main() {
    let args = DisplayArgs::parse();
    let desired_size = if args.fit {
        DisplaySize::Fit
    } else if args.width.is_none() && args.height.is_none() {
        DisplaySize::Fill
    } else {
        DisplaySize::Size(args.width, args.height)
    };

    let _ = ctrlc::set_handler(|| {
        Animation::clean();
        exit(0);
    });

    let file_in = BufReader::new(File::open(&args.file).unwrap());
    let decoder = GifDecoder::new(file_in).unwrap();
    let frames = decoder.into_frames();
    let generated: Vec<StrFrame> = frames
        .map(|f| render_frame(f.unwrap(), &desired_size))
        .collect();
    let mut anim = Animation::new(args, generated);
    anim.display();
    // display_anim(generated, &args);
}
