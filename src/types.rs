use std::time::Duration;

pub struct StrFrame {
    pub raw_frame: Vec<String>,
    pub size: (usize, usize),
    pub final_frame: Option<String>,
    pub delay: Duration,
}
