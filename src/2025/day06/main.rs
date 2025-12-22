use std::fs;

fn main() {
    let contents = fs::read_to_string("problems.txt")
        .expect("Failed to read the file");
    let lines: Vec<&str> = contents.lines().collect();
    let height = lines.len();
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let mut grand_total_pt1: u64 = 0;
    let mut x = 0;

    while x < width {
        // skip empty columns
        if lines.iter().all(|line| line.chars().nth(x).unwrap_or(' ') == ' ') {
            x += 1;
            continue;
        }

        // find end of this problem (first fully empty column)
        let mut end = x;
        while end < width && !lines.iter().all(|line| line.chars().nth(end).unwrap_or(' ') == ' ') {
            end += 1;
        }

        // last row has operator
        let op_row = height - 1;
        let operator = lines[op_row][x..end]
            .chars()
            .find(|&c| c != ' ')
            .expect("No operator found");

        // collect one number per row
        let mut nums = Vec::new();
        for y in 0..op_row {
            let row_slice: String = lines[y][x..end].chars().collect();
            let num_str = row_slice.trim().split_whitespace().next();
            if let Some(s) = num_str {
                nums.push(s.parse::<u64>().unwrap());
            }
        }

        let result = match operator {
            '+' => nums.iter().sum::<u64>(),
            '*' => nums.iter().product::<u64>(),
            _ => panic!("Unknown operator"),
        };

        grand_total_pt1 += result;
        x = end;
    }

    println!("Grand total pt1: {}", grand_total_pt1);

    // Part 2: right-to-left, vertical digits
    let mut grand_total_part2: u64 = 0;
    let mut x = 0;

    while x < width {
        if lines.iter().all(|line| line.chars().nth(x).unwrap_or(' ') == ' ') {
            x += 1;
            continue;
        }

        let mut end = x;
        while end < width && !lines.iter().all(|line| line.chars().nth(end).unwrap_or(' ') == ' ') {
            end += 1;
        }

        let op_row = height - 1;
        let operator = lines[op_row][x..end]
            .chars()
            .find(|&c| c != ' ')
            .expect("No operator found");

        let mut nums = Vec::new();
        for col in (x..end).rev() {
            let mut num_str = String::new();
            for row in 0..op_row {
                let c = lines[row].chars().nth(col).unwrap_or(' ');
                if c.is_numeric() {
                    num_str.push(c);
                }
            }
            if !num_str.is_empty() {
                nums.push(num_str.parse::<u64>().unwrap());
            }
        }

        let result = match operator {
            '+' => nums.iter().sum::<u64>(),
            '*' => nums.iter().product::<u64>(),
            _ => panic!("Unknown operator"),
        };

        grand_total_part2 += result;
        x = end;
    }

    println!("Grand total pt2: {}", grand_total_part2);
}