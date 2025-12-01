use std::fs;    // lib to read a file

fn main() {
    let contents = fs::read_to_string("rotations.txt") // reads file contents into a str
        .expect("Failed to read the file");

    let rotations: Vec<&str> = contents.lines().collect();

    let mut p1_zero_counter = 0;
    let mut p2_zero_counter = 0; // counter for part 2
    
    let mut p1_pos : i32 = 50; // starting dial position
    let mut p2_pos : i32 = 50; 

    for rotation in &rotations {
        let direction = &rotation[0..1]; // first char is always L or R
        let amount : i32 = rotation[1..].parse().unwrap(); // the rest is the rotation amount

        if direction == "L" {   // rotate left
            p1_pos = (p1_pos - amount).rem_euclid(100);  // rem_euclid handles circular rotations
        } else if direction == "R" {
            p1_pos = (p1_pos + amount).rem_euclid(100); // rotate right
        }

        if p1_pos == 0 {  //if the position is 0, update counter
            p1_zero_counter += 1; 
        }
        

         // Part 2: every full spin (100) passes 0 once
        p2_zero_counter += amount / 100;
        let remainder = amount % 100; // remainder after full spins

        for _ in 0..remainder {     // check each click of the remainder to see if it marks 0
            if direction == "R" {
                p2_pos = (p2_pos + 1).rem_euclid(100); 
            } else {
                p2_pos = (p2_pos - 1).rem_euclid(100);
            }

            if p2_pos == 0 {
                p2_zero_counter += 1;
            }
        }
    }

    println!("The password is: {}", p1_zero_counter);
    println!("Using password method 0x434C49434B, it is: {}", p2_zero_counter);

}

