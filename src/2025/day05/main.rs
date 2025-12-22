use std::fs;

fn main() {

    let contents = fs::read_to_string("ingredients.txt") // reads file contents into a str
        .expect("Failed to read the file");

    // split into ranges and available IDs
    let parts: Vec<&str> = contents.split("\n\n").collect();
    let range_lines = parts[0].lines();
    let id_lines = parts[1].lines();

    // parse ranges into a Vec of (start, end)
    let mut ranges = Vec::new();
    for line in range_lines {
        if let Some((start, end)) = line.split_once('-') {
            let start: u64 = start.parse().unwrap();
            let end: u64 = end.parse().unwrap();
            ranges.push((start, end));
        }
    }

    let ids: Vec<u64> = id_lines        // parse available IDs
        .map(|line| line.parse().unwrap())
        .collect();

    // count fresh IDs
    let mut fresh_count = 0;
    for id in ids {
        if ranges.iter().any(|&(start, end)| id >= start && id <= end) {
            fresh_count += 1;
        }
    }

    // --- Part 2 ---
    ranges.sort_unstable_by_key(|&(start, _)| start);  // sort ranges by start

    // merge overlapping/adjacent ranges
    let mut merged: Vec<(u64, u64)> = Vec::new();
    for (start, end) in ranges {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 + 1 { // overlapping or adjacent
                last.1 = last.1.max(end); // merge
            } else {
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }

    // Count total IDs covered by merged ranges
    let total_fresh_ids: u64 = merged.iter().map(|&(s, e)| e - s + 1).sum();

    println!("Number of fresh ingredient IDs is: {}", fresh_count);
    println!("Total ingredient IDs considered fresh: {}", total_fresh_ids);
}