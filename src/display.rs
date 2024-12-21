use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Instant,
};

use crate::{
    args::DisplayArgs,
    terminal_consts::{CLS_COLOR, CLS_SCREEN, HIDE_CURSOR, MOVE_CORNER, SHOW_CURSOR},
    types::{StrFrame, Vec2},
};

fn finalize_frame(f: &mut StrFrame, offset: &Vec2<usize>) {
    let mut final_frame = String::new();
    let (off_x, off_y) = (offset.0, offset.1);
    for (i, line) in f.raw_frame.iter().enumerate() {
        let line_pos = off_y + i;
        let column_pos = off_x;
        final_frame.push_str(&format!("\x1B[{line_pos};{column_pos}H{line}{CLS_COLOR}"));
    }
    f.final_frame = Some(final_frame);
}

pub fn display_anim(mut str_frames: Vec<StrFrame>, args: &DisplayArgs) {
    let mut out = std::io::stdout();
    let _ = out.write(CLS_SCREEN.as_bytes());
    let _ = out.write(HIDE_CURSOR.as_bytes());
    let mut offset = Vec2(0, 0);
    if args.center {
        if let Some(size) = termsize::get() {
            let frame_w = str_frames[0].size.0;
            let frame_h = str_frames[0].size.1;
            offset = Vec2(
                (size.cols as usize - frame_w) / 2,
                (size.rows as usize - frame_h) / 2,
            )
        }
    }
    loop {
        for f in &mut str_frames {
            if f.final_frame.is_none() {
                finalize_frame(f, &offset);
            }
            let start = Instant::now();
            // let _ = out.write(MOVE_CORNER.as_bytes());
            let _ = out.write(f.final_frame.as_ref().unwrap().as_bytes());
            let _ = out.flush();
            let end = Instant::now();
            sleep(f.delay.saturating_sub(end - start));
        }

        if !args.loop_animation {
            break;
        }
    }
    clean();
}

pub fn clean() {
    let mut out = stdout();
    let _ = out.write(CLS_SCREEN.as_bytes());
    let _ = out.write(SHOW_CURSOR.as_bytes());
    let _ = out.write(MOVE_CORNER.as_bytes());
    let _ = out.flush();
}
