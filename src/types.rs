use std::time::Duration;

pub struct StrFrame {
    pub raw_frame: Vec<String>,
    pub size: Vec2<usize>,
    pub final_frame: Option<String>,
    pub delay: Duration,
}

pub struct Vec2<T>(pub T, pub T);
