use std::fs;    // lib to read a file

fn main() {
    let contents = fs::read_to_string("rotations.txt") // reads file contents into a str
        .expect("Failed to read the file");

    let rotations: Vec<&str> = contents.lines().collect();

    let mut zero_counter = 0;
    let mut dial_position = 50; // starting dial position

    for rotation in &rotations {
        let direction = &rotation[0..1]; // first char is always L or R
        let amount : i32 = rotation[1..].parse().unwrap(); // the rest is the rotation amount

        if direction == "L" {   // rotate left
            dial_position = (dial_position - amount).rem_euclid(100);  // rem_euclid handles circular rotations
        } else if direction == "R" {
            dial_position = (dial_position + amount).rem_euclid(100); // rotate right
        }

        if dial_position == 0 {  //if the position is 0, update counter
            zero_counter += 1; 
        }
    }

    println!("The password is: {}", zero_counter);

}

