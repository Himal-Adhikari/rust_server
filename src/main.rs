use itertools::Itertools;
use rerun::{LineStrips2D, Points2D, RecordingStreamBuilder, external::glam::Vec2};
use server::handle_connection;
use std::{net::TcpListener, sync::mpsc, thread};

use crate::server::PointVector;

mod server;

fn main() {
    let listerner = TcpListener::bind("192.168.9.183:6060").unwrap();

    let (mpsc_tx, mpsc_rx) = mpsc::channel::<PointVector>();

    thread::spawn(move || {
        let mut vectors = Vec::new();
        for _ in 0..3 {
            vectors.push(PointVector::default());
        }
        let rec = RecordingStreamBuilder::new("rust_vectors_grid")
            .spawn()
            .unwrap();

        loop {
            for val in mpsc_rx.try_iter(){
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

                    let m1 = first_line.dir.y / first_line.dir.x;
                    let m2 = second_line.dir.y / second_line.dir.x;

                    let x = (m1 * first_line.origin.x - m2 * second_line.origin.x
                        + second_line.origin.y
                        - first_line.origin.y)
                        / (m1 - m2);
                    let y = m1 * x - m1 * first_line.origin.x + first_line.origin.y;
                    [x, y]
                })
                .collect::<Vec<_>>();

            let centroid_x =
                (intersections[0][0] + intersections[1][0] + intersections[2][0]) / 3.0;
            let centroid_y =
                (intersections[0][1] + intersections[1][1] + intersections[2][1]) / 3.0;

            println!("Centroid: {}, {}", centroid_x, centroid_y);

            let bounds = 10;
            let mut grid_strips = Vec::new();
            for i in -bounds..=bounds {
                let f = i as f32;
                let b = bounds as f32;

                grid_strips.push(vec![Vec2::new(f, -b), Vec2::new(f, b)]);
                grid_strips.push(vec![Vec2::new(-b, f), Vec2::new(b, f)]);
            }

            //let scale = 20.0;
            //let mut vector_strips = Vec::new();
            //let mut vector_colors = Vec::new();

            //for line in &vectors {
            //    let d = line.dir.normalize();
            //    let start = line.origin - d * scale;
            //    let end = line.origin + d * scale;

            //    vector_strips.push(vec![start, end]);
            //    vector_colors.push(rerun::Color::from_rgb(12, 23, 40));
            //}

            //rec.log(
            //    "vectors",
            //    &LineStrips2D::new(vector_strips)
            //        .with_colors(vector_colors)
            //        .with_radii([0.05]),
            //)
            //.unwrap();

            //let mut points = Vec::new();
            //for point in intersections {
            //    points.push(Vec2::new(point[0], point[1]));
            //}

            //rec.log(
            //    "intersections",
            //    &Points2D::new(points)
            //        .with_colors([rerun::Color::from_rgb(255, 255, 0)]) // Yellow
            //        .with_radii([0.2]),
            //)
            //.unwrap();

            //rec.log(
            //    "centroi",
            //    &Points2D::new(vec![Vec2::new(centroid_x, centroid_y)])
            //        .with_colors([rerun::Color::from_rgb(200, 100, 50)])
            //        .with_radii([0.2]),
            //)
            //.unwrap();

            //rec.log(
            //    "grid",
            //    &LineStrips2D::new(grid_strips)
            //        .with_colors([rerun::Color::from_rgb(80, 80, 80)])
            //        .with_radii([0.02]),
            //)
            //.unwrap();
        }
    });

    for (idx, stream) in listerner.incoming().enumerate() {
        let stream = stream.unwrap();
        let transmitter = mpsc_tx.clone();

        thread::spawn(move || {
            handle_connection(stream, transmitter, idx);
        });
    }
}
