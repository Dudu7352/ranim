use std::time::Duration;

use image::{
    buffer,
    imageops::{resize, FilterType},
    Frame, Rgba,
};

use crate::{
    args::DisplaySize,
    types::{StrFrame, Vec2},
};

fn render_row(row: buffer::Pixels<'_, Rgba<u8>>) -> String {
    let mut last_pixel_opt = None;
    let mut line = String::with_capacity(row.len() * 20);
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

pub fn render_frame(frame: Frame, desired_size: &DisplaySize) -> StrFrame {
    let (delay_num, delay_den) = frame.delay().numer_denom_ms();
    let delay = Duration::from_millis(delay_num as u64 / delay_den as u64);
    let original_buffer = frame.buffer();

    let (new_w, new_h) = match &desired_size {
        DisplaySize::Width(desired_width) => {
            let desired_height = ((original_buffer.height() as f32
                / original_buffer.width() as f32)
                * *desired_width as f32) as u32;
            (*desired_width * 2, desired_height)
        }
        DisplaySize::Fill => {
            let s = termsize::get().unwrap();
            (s.cols as u32, (s.rows - 1) as u32)
        }
    };

    let b = resize(original_buffer, new_w, new_h, FilterType::Lanczos3);
    let raw_frame: Vec<String> = b.rows().map(render_row).collect();

    StrFrame {
        raw_frame,
        final_frame: None,
        size: Vec2(b.width() as usize, b.height() as usize),
        delay,
    }
}
