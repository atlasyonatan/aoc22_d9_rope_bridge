use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod point2;
use point2::Point2;

fn main() {
    let path = Path::new("../input.txt");
    let file = File::open(path).unwrap();
    let mut set = HashSet::new();
    let start: Point2<i32> = Point2::default();
    let mut head = start;
    let mut tail = start;
    set.insert(tail);
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
            let old = head;
            head += direction;
            let dv = head - tail; //tail to head
            if dv.x.abs() > 1 || dv.y.abs() > 1 {
                tail = old;
                set.insert(tail);
            }
        }
    }

    println!("part 1: {}", set.len())
}
