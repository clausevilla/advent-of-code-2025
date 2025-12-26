use std::fs;

fn diagram_to_mask(diagram: &str) -> u32 {
    diagram
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '#')
        .map(|(i, _)| 1u32 << i)
        .fold(0, |a, b| a | b)
}

fn parse_mask(indices: &str) -> u32 {
    indices
        .split(',')
        .map(|x| 1u32 << x.parse::<u32>().unwrap())
        .fold(0, |a, b| a | b)
}

fn min_presses(target: u32, buttons: &[u32]) -> u32 {
    let n = buttons.len();
    let mut best = u32::MAX;

    for mask in 0u32..(1u32 << n) {
        let mut result = 0u32;
        let mut presses = 0;

        for i in 0..n {
            if (mask >> i) & 1 == 1 {
                result ^= buttons[i];
                presses += 1;
            }
        }

        if result == target {
            best = best.min(presses);
        }
    }

    best
}

fn main() {
    let contents = fs::read_to_string("lights.txt")
        .expect("Failed to read the file");

    let mut total_sum = 0u32;

    for line in contents.lines().filter(|l| !l.trim().is_empty()) {
        let mut parts = line.split_whitespace();

        // Indicator diagram
        let diagram = parts.next().unwrap();
        let target = diagram_to_mask(&diagram[1..diagram.len() - 1]);

        // Buttons
        let mut buttons = Vec::new();
        for part in parts {
            if part.starts_with('{') {
                break; // ignore joltage
            }
            let inside = &part[1..part.len() - 1];
            buttons.push(parse_mask(inside));
        }

        total_sum += min_presses(target, &buttons);
    }

    println!("The total output is: {}", total_sum);
}