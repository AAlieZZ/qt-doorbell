use crate::Frame;

pub trait VideoFrame {
    fn get_frame(&mut self) -> Frame;
}