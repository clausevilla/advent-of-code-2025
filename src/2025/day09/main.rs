use std::fs;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let contents = fs::read_to_string("tiles.txt")
        .expect("Failed to read the file");

    let red: Vec<Point> = contents          //parse red tiles
        .lines()
        .map(|line| {
            let mut p = line.split(',');
            Point {
                x: p.next().unwrap().parse().unwrap(),
                y: p.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    let mut part1 = 0i64;
    for i in 0..red.len() {
        for j in i + 1..red.len() {
            let w = (red[i].x - red[j].x).abs() as i64 + 1;
            let h = (red[i].y - red[j].y).abs() as i64 + 1;
            part1 = part1.max(w * h);
        }
    }

    println!("Part 1 output: {}", part1);

}