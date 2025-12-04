use std::fs;


fn invalid_id_p1(number:u64) -> bool {
    let id = number.to_string();
    let length = id.len();

    if length % 2 != 0 {        // check if its not even
        return false;
    }

    let half = length / 2;
    &id[..half] == &id[half..]
}

fn invalid_id_p2(number:u64) -> bool {
    let id = number.to_string();
    let length = id.len();

    for k in 1..=(length / 2) {  // pattern length must be less or equal than half the number
        if length % k != 0 {
            continue;       // must evenly divide the length
        }

        let pattern = &id[..k];
        let repeated = pattern.repeat(length / k);
        if repeated == id {
            return true;
        }
    }
    false
}


fn main() {
    let contents = fs::read_to_string("ids.txt") // reads file contents into a str
        .expect("Failed to read the file");

    let mut total_sum_p1 = 0; // sum of invalid ids
    let mut total_sum_p2 = 0;

    for range in contents.trim().split(',') {
        if range.is_empty() {
            continue;
        }

        let (start, end) = range    //split the id string into start and end
            .split_once('-')
            .expect("Invalid range format");

        let start: u64 = start.parse().unwrap();
        let end: u64 = end.parse().unwrap();

        for id in start..=end {     //sum up invalid ids
            if invalid_id_p1(id) { total_sum_p1 += id; }
            if invalid_id_p2(id) { total_sum_p2 += id; }
        }
    }

    println!("The total sum is: {}", total_sum_p1);
    println!("In part 2, the sum is: {}", total_sum_p2);

}