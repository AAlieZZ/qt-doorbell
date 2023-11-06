use std::net::UdpSocket;
use crate::{Frame, video_frame::VideoFrame};

pub struct UdpReader {
	socket: UdpSocket,
	data: Vec<u8>,
}

impl VideoFrame for UdpReader {
	fn get_frame(&mut self) -> Frame {
		let mut all: Vec<u8> = Vec::new();
		let mut number_of_bytes: usize = 1472;
		while number_of_bytes == 1472 {
			let mut buf: [u8; 1472] = [0; 1472];
			match self.socket.recv_from(&mut buf) {
				Ok(n) => number_of_bytes = n.0,
				Err(_) => number_of_bytes = 0,
			}
			all.extend(buf);
		}
		let (c, r) = from_bytes(&all[..8].try_into().unwrap());
		self.data = Vec::from(&all[8..]);
		Frame {
			data: self.data.as_ptr(),
			cols: c,
			rows: r,
		}
	}
}

impl UdpReader {
	pub fn udp_reader() -> UdpReader {
		let s = UdpSocket::bind("0.0.0.0:4000").expect("couldn't bind to address");
		UdpReader { socket: s, data: Vec::new() }
	}
}

fn from_bytes(buf: &[u8; 8]) -> (i32, i32) {
	let cols_rows: Vec<i32> = buf.map(|u| u as i32).chunks(4).map(|b| (b[0] as i32 * 1000) + (b[1] * 100) + (b[2] * 10) + b[3]).collect();
	(cols_rows[0], cols_rows[1])
}