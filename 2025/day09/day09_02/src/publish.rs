use crate::point::Point;

use std::cmp;
use std::fmt::Write as FMTWrite;
use std::fs::File;
use std::io::{BufWriter, Write as IOWrite};

fn max_shape_extent(shape: &Vec<Point>) -> i64 {
    let mut max_extent = -1;
    for point in shape.iter() {
        max_extent = cmp::max(max_extent, max_coord(point));
    }
    max_extent
}

fn max_coord(point: &Point) -> i64 {
    cmp::max(point.x, point.y)
}

pub fn publish_shape(filename: &str, shape: &Vec<Point>, rect: &Vec<Point>) {
    let file = File::create(filename).unwrap();
    let mut writer = BufWriter::new(file);

    let extent = {
        let shp_max = max_shape_extent(shape);
        let rect_max = max_shape_extent(rect);
        cmp::max(shp_max, rect_max) as f64
    };

    writeln!(
        writer,
        "<svg width=\"500\" height=\"500\" xmlns=\"http://www.w3.org/2000/svg\">"
    )
    .unwrap();

    let scale = |x: i64| (x as f64) * 500.0 / extent;

    {
        let mut points_str = String::new();
        for point in shape.iter() {
            write!(points_str, "{},{} ", scale(point.x), scale(point.y)).unwrap();
        }

        writeln!(
            writer,
            "<polygon fill=\"lightblue\" stroke=\"black\" points=\"{}\"/>",
            points_str
        )
        .unwrap();
    }

    for point in shape.iter() {
        writeln!(
            writer,
            "<circle fill=\"black\" stroke=\"none\" r=\"2\" cx=\"{}\" cy=\"{}\" />",
            scale(point.x),
            scale(point.y)
        )
        .unwrap();
    }

    {
        let mut points_str = String::new();
        for point in rect.iter() {
            write!(points_str, "{},{} ", scale(point.x), scale(point.y)).unwrap();
        }

        writeln!(
            writer,
            "<polygon fill=\"rgba(250, 100, 100, 0.5)\" points=\"{}\"/>",
            points_str
        )
        .unwrap();
    }

    writeln!(writer, "</svg>").unwrap();
}
