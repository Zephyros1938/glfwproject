use crate::util::event_args::EventArgs;

#[derive(Copy, Clone, Debug)]
pub struct FrameEventArgs {
    pub time: f32,
    pub total_time: f32,
}

impl FrameEventArgs {
    pub fn new(time: f32, total_time: f32) -> Self {
        Self { time, total_time }
    }
}

impl EventArgs for FrameEventArgs {}
