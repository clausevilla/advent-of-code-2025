use std::fs;

fn main() {

    let contents = fs::read_to_string("rolls.txt") // reads file contents into a str
        .expect("Failed to read the file");

    let mut grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut accessible_count = 0;
    let mut total_removed = 0; // part 2

    // directions for the 8 neighbors
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),          (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '@' {
                let mut adjacent_count = 0;
                for (dr, dc) in directions.iter() {
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;
                    if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                        if grid[nr as usize][nc as usize] == '@' {
                            adjacent_count += 1;
                        }
                    }
                }
                if adjacent_count < 4 {
                    accessible_count += 1;
                }
            }
        }
    }

    loop {  // part 2
        let mut to_remove = vec![];

        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == '@' {
                    let mut neighbors = 0;
                    for ni in i.saturating_sub(1)..=(i+1).min(rows-1) {
                        for nj in j.saturating_sub(1)..=(j+1).min(cols-1) {
                            if ni == i && nj == j { continue; }
                            if grid[ni][nj] == '@' {
                                neighbors += 1;
                            }
                        }
                    }
                    if neighbors < 4 {
                        to_remove.push((i, j));
                    }
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for (i, j) in to_remove.iter() {
            grid[*i][*j] = '.';
        }

        total_removed += to_remove.len();
    }

    println!("The total accessible rolls of paper: {}", accessible_count);
    println!("Total rolls removed: {}", total_removed);
}