use std::cmp;
use std::io::{self, Read};
use std::iter::FromIterator;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let parsed: Vec<&str> = buffer.split_whitespace().collect();
    let wire1_path: Vec<(char, i64)> = parsed[0].trim().split(',').map(parse_segment).collect();
    let wire2_path: Vec<(char, i64)> = parsed[1].trim().split(',').map(parse_segment).collect();
    let mut intersections = Vec::new();

    let mut wire1_current_start = (1, 1);
    for wire1_segment in wire1_path {
        let wire1_current_end = add_segment_to_point(wire1_current_start, wire1_segment);
        let mut wire2_current_start = (1, 1);
        for wire2_segment in &wire2_path {
            let wire2_current_end = add_segment_to_point(wire2_current_start, *wire2_segment);
            if let Some(intersection) = get_intersection(
                (wire1_current_start, wire1_current_end),
                (wire2_current_start, wire2_current_end),
            ) {
                if intersection != (1, 1) {
                    intersections.push(intersection);
                }
            }
            wire2_current_start = wire2_current_end;
        }
        wire1_current_start = wire1_current_end;
    }

    let mut shortest: Vec<i64> = intersections
        .iter()
        .map(|(x, y)| (x.abs() - 1) + (y.abs() - 1))
        .collect();
    shortest.sort();

    println!("{}", shortest[0]);
    Ok(())
}

fn parse_segment(s: &str) -> (char, i64) {
    let chars: Vec<char> = s.chars().collect();
    let direction = chars[0];
    let magnitude = String::from_iter(&chars[1..]).parse::<i64>().unwrap();
    (direction, magnitude)
}

fn add_segment_to_point(point: (i64, i64), segment: (char, i64)) -> (i64, i64) {
    match segment {
        ('R', magnitude) => (point.0 + magnitude, point.1),
        ('L', magnitude) => (point.0 - magnitude, point.1),
        ('U', magnitude) => (point.0, point.1 + magnitude),
        ('D', magnitude) => (point.0, point.1 - magnitude),
        _ => panic!("invalid direction"),
    }
}

fn get_intersection(
    line1: ((i64, i64), (i64, i64)),
    line2: ((i64, i64), (i64, i64)),
) -> Option<(i64, i64)> {
    let a = line1.0;
    let b = line1.1;
    let a1 = b.1 - a.1;
    let b1 = a.0 - b.0;
    let c1 = a1 * a.0 + b1 * a.1;

    let c = line2.0;
    let d = line2.1;
    let a2 = d.1 - c.1;
    let b2 = c.0 - d.0;
    let c2 = a2 * c.0 + b2 * c.1;

    let det = a1 * b2 - a2 * b1;
    if det == 0 {
        None
    } else {
        let x = (b2 * c1 - b1 * c2) / det;
        let y = (a1 * c2 - a2 * c1) / det;

        let point = (x, y);
        if point_is_on_line(point, line1) && point_is_on_line(point, line2) {
            Some(point)
        } else {
            None
        }
    }
}

fn point_is_on_line(point: (i64, i64), line: ((i64, i64), (i64, i64))) -> bool {
    let line_a = line.0;
    let line_b = line.1;
    cmp::max(line_a.0, line_b.0) >= point.0
        && cmp::min(line_a.0, line_b.0) <= point.0
        && cmp::max(line_a.1, line_b.1) >= point.1
        && cmp::min(line_a.1, line_b.1) <= point.1
}
