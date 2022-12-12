use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod point2;
use point2::Point2;

fn main() {
    let path = Path::new("../input.txt");
    let file = File::open(path).unwrap();

    let start: Point2<i32> = Point2::default();
    let mut rope = vec![start; 10];
    let mut set = HashSet::new();
    set.insert(*rope.last().unwrap());

    for line in io::BufReader::new(file).lines().map(|l| l.unwrap()) {
        let mut words = line.split_ascii_whitespace();
        let direction = Point2::from(match words.next().unwrap() {
            "R" => Point2 { x: 1, y: 0 },
            "L" => Point2 { x: -1, y: 0 },
            "U" => Point2 { x: 0, y: -1 },
            "D" => Point2 { x: 0, y: 1 },
            other => panic!("unknown direction '{}'", other),
        });
        let count = match words.next() {
            Some(s) => s.parse::<usize>().unwrap(),
            None => 1,
        };
        for _ in 0..count {
            rope[0] += direction;
            for i in 1..rope.len() {
                let mut dv = rope[i] - rope[i - 1]; //head to tail
                let len = ((dv.x * dv.x + dv.y * dv.y) as f64).sqrt();
                dv.x = (dv.x as f64 / len).round() as i32;
                dv.y = (dv.y as f64 / len).round() as i32;
                rope[i] = rope[i - 1] + dv;
            }
            set.insert(*rope.last().unwrap());
        }
    }

    println!("{}", set.len())
}
