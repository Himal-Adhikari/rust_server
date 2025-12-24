use csv::Reader;
use rerun::external::glam::Vec2;
use serde::Deserialize;
use std::io::BufRead;
use std::{io::BufReader, net::TcpStream, sync::mpsc::Sender};

#[derive(Debug, Deserialize)]
pub struct RecevData {
    _timestamp: u64,
    h: f64,
    k: f64,
    phi: f64,
    mic_dis: f64,
    del_t: f64,
}

#[derive(Default)]
pub struct PointVector {
    pub idx: usize,
    pub origin: Vec2,
    pub dir: Vec2,
}

const SPEED_SOUND: f64 = 343.0;

impl PointVector {
    pub fn new(idx: usize, h: f64, k: f64, x1: f64, y1: f64) -> Self {
        let origin = Vec2::new(h as f32, k as f32);
        let dir = Vec2::new(x1 as f32, y1 as f32);
        PointVector { idx, origin, dir }
    }
}

pub fn handle_connection(stream: TcpStream, sender: Sender<PointVector>, idx: usize) {
    let mut buf_reader = BufReader::new(&stream);

    let mut buf = String::new();

    buf_reader.read_line(&mut buf).unwrap();

    buf = buf.trim().to_string();

    let mut csv_reader = Reader::from_reader(buf.as_bytes());

    for val in csv_reader.deserialize::<RecevData>() {
        let val = val.unwrap();
        let theta = ((val.del_t * SPEED_SOUND) / val.mic_dis).asin();
        let a = PointVector::new(
            idx,
            val.h,
            val.k,
            (theta + val.phi).cos(),
            (theta + val.phi).sin(),
        );
        sender.send(a).unwrap();
    }
}
