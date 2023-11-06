mod cvmat;
mod udp_reader;
mod video_frame;

use opencv::{prelude::*, videoio};
use std::net::UdpSocket;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use cvmat::CvMat;
use udp_reader::UdpReader;
use video_frame::VideoFrame;

static SENDING: AtomicBool = AtomicBool::new(false);

#[repr(C)]
pub struct Frame {
	data: *const u8,
	cols: i32,
	rows: i32,
}

#[no_mangle]
pub extern "C" fn new_cam() -> *mut CvMat {
	Box::into_raw(Box::new(CvMat::new_camera()))
}

#[no_mangle]
pub extern "C" fn get_frame(ptr: *mut CvMat) -> Frame {
	let mat = unsafe {
		assert!(!ptr.is_null());
		&mut *ptr
	};
	mat.get_frame()
}

#[no_mangle]
pub extern "C" fn del_cam(ptr: *mut CvMat) {
	if ptr.is_null() {
		return;
	} else {
		let _ = unsafe{Box::from_raw(ptr)};
	}
}

#[no_mangle]
pub extern "C" fn udp_reader() -> *mut UdpReader {
	Box::into_raw(Box::new(UdpReader::udp_reader()))
}

#[no_mangle]
pub extern "C" fn udp_frame(ptr: *mut UdpReader) -> Frame {
	let mat = unsafe {
		assert!(!ptr.is_null());
		&mut *ptr
	};
	mat.get_frame()
}

#[no_mangle]
pub extern "C" fn del_udp(ptr: *mut UdpReader) {
	if ptr.is_null() {
		return;
	} else {
		let _ = unsafe{Box::from_raw(ptr)};
	}
}

#[no_mangle]
pub extern "C" fn send_video() {
	let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY).unwrap(); // 0 is the default camera
	let opened = videoio::VideoCapture::is_opened(&cam).unwrap();
	if !opened {
		panic!("Unable to open default camera!");
	}
	SENDING.store(true, Ordering::Relaxed);
	thread::spawn(move || {
		while SENDING.load(Ordering::Relaxed) {
			let mut frame = Mat::default();
			cam.read(&mut frame).unwrap();
			let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");
			let mut crd = into_bytes(frame.cols(), frame.rows());
			crd.extend(frame.data_bytes().unwrap());
			crd.chunks(1472).for_each(|d| {
				socket.send_to(d, "192.168.1.2:4000").expect("couldn't send data");
			});
		}
	});
}

#[no_mangle]
pub extern "C" fn over() {
	SENDING.store(false, Ordering::Relaxed)
}

#[no_mangle]
pub extern "C" fn is_sending() -> bool {
	SENDING.load(Ordering::Relaxed)
}

fn into_bytes(cols: i32, rows: i32) -> Vec<u8> {
	let mut i = 100000000;
	[0; 8].into_iter().map(|_| {
		i /= 10;
		((cols * 10000 + rows) / i) as u8
	}).collect()
}
