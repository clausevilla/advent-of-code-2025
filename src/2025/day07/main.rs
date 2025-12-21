use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("manifold.txt")
        .expect("Failed to read file");
    let lines: Vec<&str> = contents.lines().collect();
    let height = lines.len();
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // Convert to 2D grid
    let mut grid = vec![vec![' '; width]; height];
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[y][x] = c;
        }
    }

    // Part 1: total beam splits
    let mut beams = HashSet::new();
    for x in 0..width {
        for y in 0..height {
            if grid[y][x] == 'S' {      // Find starting S positions
                beams.insert((x, y + 1));
            }
        }
    }

    let mut split_count = 0;

    while !beams.is_empty() {
        let mut new_beams = HashSet::new();
        for &(x, y) in &beams {
            if y >= height {
                continue;
            }
            match grid[y][x] {
                '^' => {
                    split_count += 1;
                    if x > 0 {
                        new_beams.insert((x - 1, y + 1));
                    }
                    if x + 1 < width {
                        new_beams.insert((x + 1, y + 1));
                    }
                }
                '.' => {
                    new_beams.insert((x, y + 1));
                }
                _ => {}
            }
        }
        beams = new_beams;
    }
    println!("Total beam splits: {}", split_count);

    // Part 2: quantum timelines using DP
    let mut timeline_counts = vec![vec![0u64; width]; height];

    // Initialize start positions
    for x in 0..width {
        for y in 0..height {
            if grid[y][x] == 'S' {
                timeline_counts[y + 1][x] = 1; // single particle starts below S
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            let count = timeline_counts[y][x];
            if count == 0 {
                continue;
            }
            match grid[y][x] {
                '.' | 'S' => {
                    if y + 1 < height {
                        timeline_counts[y + 1][x] += count; // propagate down
                    }
                }
                '^' => {
                    if x > 0 && y + 1 < height {
                        timeline_counts[y + 1][x - 1] += count; // left split
                    }
                    if x + 1 < width && y + 1 < height {
                        timeline_counts[y + 1][x + 1] += count; // right split
                    }
                }
                _ => {}
            }
        }
    }

    // count timelines at positions where the particle can no longer move
    let mut total_timelines: u64 = 0;
    for y in 0..height {
        for x in 0..width {
            let c = grid[y][x];
            if timeline_counts[y][x] == 0 {
                continue;
            }
            let blocked_below = y + 1 >= height || (c != '^' && c != '.' && c != 'S');
            let no_split_down = c == '^' && 
                ((x == 0 || y + 1 >= height) && (x + 1 >= width || y + 1 >= height));
            if blocked_below || no_split_down {
                total_timelines += timeline_counts[y][x];
            }
        }
    }

    println!("Part 2 — Total quantum timelines: {}", total_timelines);
}