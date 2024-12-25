use std::time::Duration;

use image::{
    imageops::{resize, FilterType},
    Frame, ImageBuffer, Rgba,
};

use crate::{args::DisplaySize, types::StrFrame};

pub fn render_line(
    buffer: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    width: usize,
    y: usize,
    render_bottom: bool,
) -> String {
    let mut last_fg = (0, 0, 0);
    let mut last_bg = (0, 0, 0);

    let mut line: String = String::with_capacity(width * 20);
    for x in 0..width {
        let pixel = buffer.get_pixel(x as u32, y as u32);
        let [r, g, b, _] = pixel.0;
        if (r, g, b) != last_fg || x == 0 {
            line.push_str(&format!("\x1b[38;2;{r};{g};{b}m"));
        }
        last_fg = (r, g, b);

        if render_bottom {
            let bottom_pixel = buffer.get_pixel(x as u32, (y + 1) as u32);
            let [next_r, next_g, next_b, _] = bottom_pixel.0;
            if (next_r, next_g, next_b) != last_bg || x == 0 {
                line.push_str(&format!("\x1b[48;2;{next_r};{next_g};{next_b}m"));
            }
            last_bg = (next_r, next_g, next_b);
        }

        line.push('▀');
    }
    line.push_str("\x1b[0m");
    line
}

pub fn to_resized_buffer(
    frame: Frame,
    desired_size: &DisplaySize,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let original_buffer = frame.buffer();
    let (new_w, new_h) = match &desired_size {
        DisplaySize::Width(desired_width) => {
            let desired_height = ((original_buffer.height() as f32
                / original_buffer.width() as f32)
                * *desired_width as f32) as u32;
            (*desired_width, desired_height)
        }
        DisplaySize::Fill => {
            let s = termsize::get().unwrap();
            (s.cols as u32, (s.rows - 1) as u32)
        }
    };
    resize(original_buffer, new_w, new_h, FilterType::Lanczos3)
}

pub fn render_frame(frame: Frame, desired_size: &DisplaySize) -> StrFrame {
    let (delay_num, delay_den) = frame.delay().numer_denom_ms();
    let delay = Duration::from_millis(delay_num as u64 / delay_den as u64);

    let buffer = to_resized_buffer(frame, desired_size);
    let (width, height) = (buffer.width() as usize, buffer.height() as usize);

    let mut result = Vec::with_capacity(width);
    for y in (0..(height - 1)).step_by(2) {
        result.push(render_line(&buffer, width, y, true));
    }

    if height % 2 == 1 {
        result.push(render_line(&buffer, width, height - 1, false));
    }

    StrFrame {
        raw_frame: result,
        final_frame: None,
        size: (width, height / 2),
        delay,
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use image::{Delay, Frame, ImageBuffer, Rgba};

    use crate::args::DisplaySize;

    use super::{render_frame, render_line};

    type I = ImageBuffer<Rgba<u8>, Vec<u8>>;

    fn prepare_black_img(size: (u32, u32)) -> I {
        let (w, h) = size;
        let mut raw: Vec<u8> = Vec::new();
        for _ in 0..(w * h) {
            raw.extend_from_slice(&[0, 0, 0, 0]);
        }
        ImageBuffer::from_raw(w, h, raw).unwrap()
    }

    fn grayscale_img(size: (u32, u32), colors: &[u8]) -> I {
        let (w, h) = size;
        let mut raw: Vec<u8> = Vec::new();
        for c in colors {
            raw.extend_from_slice(&[*c, *c, *c, *c]);
        }
        ImageBuffer::from_raw(w, h, raw).unwrap()
    }

    #[test]
    fn test_render_line_one_pixel_row() {
        let img: I = prepare_black_img((4, 5));
        let res = render_line(&img, 4, 0, false);
        let exp = "\x1b[38;2;0;0;0m▀▀▀▀\x1b[0m".to_string();
        assert_eq!(res, exp, "Invalid line render result");
    }

    #[test]
    fn test_render_line_two_pixel_rows() {
        let img: I = prepare_black_img((4, 5));
        let res = render_line(&img, 4, 0, true);
        let exp = "\x1b[38;2;0;0;0m\x1b[48;2;0;0;0m▀▀▀▀\x1b[0m".to_string();
        assert_eq!(res, exp, "Invalid line render result");
    }

    #[test]
    fn test_render_line_with_top_row_change() {
        let img: I = grayscale_img((4, 2), &[0, 0, 1, 1, 2, 2, 2, 2]);
        let res = render_line(&img, 4, 0, true);
        let exp = "\x1b[38;2;0;0;0m\x1b[48;2;2;2;2m▀▀\x1b[38;2;1;1;1m▀▀\x1b[0m".to_string();
        assert_eq!(res, exp, "Invalid line render result");
    }

    #[test]
    fn test_render_line_with_bottom_row_change() {
        let img: I = grayscale_img((4, 2), &[0, 0, 0, 0, 2, 2, 1, 1]);
        let res = render_line(&img, 4, 0, true);
        let exp = "\x1b[38;2;0;0;0m\x1b[48;2;2;2;2m▀▀\x1b[48;2;1;1;1m▀▀\x1b[0m".to_string();
        assert_eq!(res, exp, "Invalid line render result");
    }

    /// Test rendering an image of size 3x3. The result should have 2 lines, one only with top
    /// chars, since each line can contain up to 2 rows of pixels
    #[test]
    fn test_render_3x3_frame() {
        let img: I = grayscale_img((3, 3), &[0, 0, 0, 1, 1, 1, 2, 2, 2]);
        let frame: Frame = Frame::from_parts(
            img,
            0,
            0,
            Delay::from_saturating_duration(Duration::from_millis(20)),
        );

        let res = render_frame(frame, &DisplaySize::Width(3));
        let exp = vec![
            "\x1b[38;2;0;0;0m\x1b[48;2;1;1;1m▀▀▀\x1b[0m".to_string(),
            "\x1b[38;2;2;2;2m▀▀▀\x1b[0m".to_string(),
        ];

        assert_eq!(res.raw_frame, exp);
    }

    /// Test rendring an image of size 1x4. The result should have 2 lines, since each line can
    /// contain up to 2 rows of pixels
    #[test]
    fn test_render_1x4_frame() {
        let img: I = grayscale_img((1, 4), &[0, 1, 2, 3]);
        let frame: Frame = Frame::from_parts(
            img,
            0,
            0,
            Delay::from_saturating_duration(Duration::from_millis(20)),
        );

        let res = render_frame(frame, &DisplaySize::Width(1));
        let exp = vec![
            "\x1b[38;2;0;0;0m\x1b[48;2;1;1;1m▀\x1b[0m".to_string(),
            "\x1b[38;2;2;2;2m\x1b[48;2;3;3;3m▀\x1b[0m".to_string(),
        ];

        assert_eq!(res.raw_frame, exp);
    }
}
