use opencv::{prelude::*, videoio};
use crate::{Frame, video_frame::VideoFrame};

pub struct CvMat {
	cam: videoio::VideoCapture
}

impl VideoFrame for CvMat {
	fn get_frame(&mut self) -> Frame {
		let opened = videoio::VideoCapture::is_opened(&self.cam).unwrap();
		if !opened {
			panic!("Unable to open default camera!");
		}
		let mut frame = Mat::default();
		self.cam.read(&mut frame).unwrap();
		Frame {
			data: frame.data(),
			cols: frame.cols(),
			rows: frame.rows(),
		}
	}
}

impl CvMat {
    pub fn new_camera() -> CvMat {
        let c = videoio::VideoCapture::new(0, videoio::CAP_ANY).unwrap(); // 0 is the default camera
		let opened = videoio::VideoCapture::is_opened(&c).unwrap();
		if !opened {
			panic!("Unable to open default camera!");
		}
		CvMat { cam: c }
    }

	// pub fn del(self) {
	// 	if self.raw.is_null() {
	// 		return;
	// 	} else {
	// 		unsafe {Mat::from_raw(self.raw);}
	// 	}
	// }
}