use std::fs;

fn max_joltage_1(line: &str) -> u32 {     // part 1
    let digits: Vec<u32> = line
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let mut max = 0;

    for i in 0..digits.len() {
        for j in (i + 1)..digits.len() {
            let value = digits[i] * 10 + digits[j];
            if value > max {
                max = value;
            }
        }
    }
    max
}


fn max_joltage_2(line: &str) -> u128 {
    let digits: Vec<u32> = line
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let mut stack: Vec<u32> = Vec::new();

    // number of digits we are allowed to remove
    let mut to_remove = digits.len() - 12;

    for &d in &digits {
        while to_remove > 0     //while stack is not empty, current digit > last kept digit
            && !stack.is_empty()
            && *stack.last().unwrap() < d
        {
            stack.pop();       // remove smaller digit
            to_remove -= 1;    // count one removal
        }

        stack.push(d);         // keep current digit
    }

    // if we still need to remove digits, remove from the end
    stack.truncate(12);

    // convert into a single number
    let mut value: u128 = 0;
    for d in stack {
        value = value * 10 + d as u128;
    }

    value
}


fn main() {

    let contents = fs::read_to_string("batteries.txt") // reads file contents into a str
        .expect("Failed to read the file");


    let total_sum_1: u32 = contents
        .lines()
        .map(max_joltage_1)
        .sum();

    let total_sum_2: u128 = contents
        .lines()
        .map(max_joltage_2)
        .sum();

    println!("The total output joltage for pt1 is: {}", total_sum_1);
    println!("The total output joltage for pt2 is: {}", total_sum_2);
}