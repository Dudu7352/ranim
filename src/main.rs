use std::{
    fs::File,
    io::{stdout, BufReader, Write},
    process::exit,
    thread::sleep,
    time::{Duration, Instant},
};

use args::{DisplayArgs, DisplaySize};
use clap::Parser;
use image::{
    buffer,
    codecs::gif::GifDecoder,
    imageops::{resize, FilterType},
    AnimationDecoder, Frame, Rgba,
};

mod args;

const MOVE_CORNER: &'static str = "\x1B[H";
const CLS_COLOR: &'static str = "\x1B[0m";
const CLS_SCREEN: &'static str = "\x1B[2J";
const HIDE_CURSOR: &'static str = "\x1b[?25l";
const SHOW_CURSOR: &'static str = "\x1b[?25h";

struct StrFrame {
    pub raw_frame: Vec<String>,
    pub final_frame: Option<String>,
    pub delay: Duration,
}

fn render_row(row: buffer::Pixels<'_, Rgba<u8>>) -> String {
    let mut last_pixel_opt = None;
    let mut line = String::with_capacity(row.len() as usize * 20);
    for p in row {
        if let Some(last_pixel) = last_pixel_opt {
            if last_pixel == p {
                line.push(' ');
                continue;
            }
        }
        let [r, g, b, _a] = p.0; // I am the one who ~~knocks~~ ignores the alpha channel

        line.push_str(&format!("\x1B[48;2;{r};{g};{b}m "));
        last_pixel_opt = Some(p);
    }
    line
}

fn render_frame(frame: Frame, desired_size: &DisplaySize) -> StrFrame {
    let (delay_num, delay_den) = frame.delay().numer_denom_ms();
    let delay = Duration::from_millis(delay_num as u64 / delay_den as u64);
    let original_buffer = frame.buffer();

    let (new_w, new_h) = match &desired_size {
        DisplaySize::Width(desired_width) => {
            let desired_height = ((original_buffer.height() as f32 / original_buffer.width() as f32) * *desired_width as f32) as u32;
            (*desired_width, desired_height)
        },
        DisplaySize::Fill => {
            let s = termsize::get().unwrap();
            (s.cols as u32, (s.rows - 1) as u32)
        }
    };


    let b = resize(
        original_buffer,
        new_w * 2,
        new_h,
        FilterType::Lanczos3,
    );
    let raw_frame: Vec<String> = b.rows().map(render_row).collect();

    StrFrame {
        raw_frame,
        final_frame: None,
        delay,
    }
}

fn finalize_frame(f: &mut StrFrame) {
    let mut final_frame = String::new();
    for line in f.raw_frame.iter() {
        final_frame.push_str(&format!("{line}{CLS_COLOR}\n"));
    }
    f.final_frame = Some(final_frame);
}

fn display(mut str_frames: Vec<StrFrame>, loop_animation: bool) {
    let mut out = std::io::stdout();
    let _ = out.write(CLS_SCREEN.as_bytes());
    let _ = out.write(HIDE_CURSOR.as_bytes());
    loop {
        for f in &mut str_frames {
            if f.final_frame.is_none() {
                finalize_frame(f);
            }
            let start = Instant::now();
            let _ = out.write(MOVE_CORNER.as_bytes());
            let _ = out.write(f.final_frame.as_ref().unwrap().as_bytes());
            let _ = out.flush();
            let end = Instant::now();
            sleep(f.delay.saturating_sub(end - start));
        }

        if !loop_animation {
            break;
        }
    }
    clean();
}

fn clean() {
    let mut out = stdout();
    let _ = out.write(CLS_SCREEN.as_bytes());
    let _ = out.write(SHOW_CURSOR.as_bytes());
    let _ = out.write(MOVE_CORNER.as_bytes());
    let _ = out.flush();
}

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

    let file_in = BufReader::new(File::open(args.file).unwrap());
    let decoder = GifDecoder::new(file_in).unwrap();
    let frames = decoder.into_frames();
    let generated: Vec<StrFrame> = frames
        .map(|f| render_frame(f.unwrap(), &desired_size))
        .collect();
    display(generated, args.loop_animation);
}
