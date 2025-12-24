use itertools::Itertools;
use server::handle_connection;
use std::{net::TcpListener, sync::mpsc, thread};

use crate::server::PointVector;

mod server;

fn main() {
    let listerner = TcpListener::bind("127.0.0.0:6969").unwrap();

    let (mpsc_tx, mpsc_rx) = mpsc::channel();

    for (idx, stream) in listerner.incoming().enumerate() {
        let stream = stream.unwrap();
        let transmitter = mpsc_tx.clone();

        thread::spawn(move || {
            handle_connection(stream, transmitter, idx);
        });
    }

    thread::spawn(move || {
        let mut vectors = Vec::new();
        for _ in 0..3 {
            vectors.push(PointVector::default());
        }
        loop {
            if let Ok(val) = mpsc_rx.try_recv() {
                match val.idx {
                    0 => {
                        vectors[0] = val;
                    }
                    1 => {
                        vectors[1] = val;
                    }
                    2 => {
                        vectors[2] = val;
                    }
                    _ => unreachable!(),
                }
            }

            let intersections = vectors
                .iter()
                .combinations(2)
                .map(|combination| {
                    let first_line = combination[0];
                    let second_line = combination[1];

                    let m1 = first_line.y1 / first_line.x1;
                    let m2 = second_line.y1 / second_line.x1;

                    let x = (m1 * first_line.h - m2 * second_line.h + second_line.k - first_line.k)
                        / (m1 - m2);
                    let y = m1 * x - m1 * first_line.h + first_line.k;
                    [x, y]
                })
                .collect::<Vec<_>>();

            let x = (intersections[0][0] + intersections[1][0] + intersections[2][0]) / 3.0;
            let y = (intersections[0][1] + intersections[1][1] + intersections[2][1]) / 3.0;

            println!("{}, {}", x, y);
        }
    });
}
