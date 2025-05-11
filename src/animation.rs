use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Instant,
};

use crate::{
    args::DisplayArgs,
    terminal_consts::{CLS_COLOR, CLS_SCREEN, HIDE_CURSOR, MOVE_CORNER, SHOW_CURSOR},
    types::StrFrame,
};

pub struct Animation {
    args: DisplayArgs,
    frames: Vec<StrFrame>,
}

impl Animation {
    const SPACES_BETWEEN_ANIM_AND_FRAME: usize = 1;

    pub fn new(args: DisplayArgs, frames: Vec<StrFrame>) -> Self {
        Self { args, frames }
    }

    pub fn display(&mut self) {
        let mut out = std::io::stdout();
        let _ = out.write(CLS_SCREEN.as_bytes());
        let _ = out.write(HIDE_CURSOR.as_bytes());
        let mut offset = (1, 1);
        if self.args.center {
            if let Some(size) = termsize::get() {
                let mut frame_w = self.frames[0].size.0;
                let frame_h = self.frames[0].size.1;
                if self.args.text.is_some() {
                    frame_w += Animation::SPACES_BETWEEN_ANIM_AND_FRAME;
                    frame_w += self.longest_text_line();
                }
                offset = (
                    (size.cols as usize - frame_w) / 2 + 1,
                    (size.rows as usize - frame_h) / 2 + 1,
                )
            }
        }
        if let Some(text) = &self.args.text {
            let finalized_text = Animation::finalize_text(text, &offset, self.frames[0].size.0);
            let _ = out.write(finalized_text.as_bytes());
            let _ = out.flush();
        }
        loop {
            for f in &mut self.frames {
                if f.final_frame.is_none() {
                    Animation::finalize_frame(f, &offset);
                }
                let start = Instant::now();
                let _ = out.write(f.final_frame.as_ref().unwrap().as_bytes());
                let _ = out.flush();
                let end = Instant::now();
                sleep(f.delay.saturating_sub(end - start));
            }

            if !self.args.loop_animation {
                break;
            }
        }
        Animation::clean();
    }

    fn longest_text_line(&self) -> usize {
        if let Some(text) = &self.args.text {
            return text.lines().map(|line| line.len()).max().unwrap_or(0);
        }
        0
    }

    fn finalize_text(text: &str, offset: &(usize, usize), frame_width: usize) -> String {
        let column_pos = offset.0 + Animation::SPACES_BETWEEN_ANIM_AND_FRAME + frame_width;
        text.lines()
            .enumerate()
            .map(|(line_idx, line)| format!("\x1B[{};{}H{}", offset.1 + line_idx, column_pos, line))
            .collect()
    }

    fn finalize_frame(f: &mut StrFrame, offset: &(usize, usize)) {
        let mut final_frame = String::new();
        let (off_x, off_y) = (offset.0, offset.1);
        for (i, line) in f.raw_frame.iter().enumerate() {
            let line_pos = off_y + i;
            let column_pos = off_x;
            final_frame.push_str(&format!("\x1B[{line_pos};{column_pos}H{line}{CLS_COLOR}"));
        }
        f.final_frame = Some(final_frame);
    }

    pub fn clean() {
        let mut out = stdout();
        let _ = out.write(CLS_SCREEN.as_bytes());
        let _ = out.write(SHOW_CURSOR.as_bytes());
        let _ = out.write(MOVE_CORNER.as_bytes());
        let _ = out.flush();
    }
}
