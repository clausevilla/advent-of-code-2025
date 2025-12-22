use std::fs;




fn main() {

    let contents = fs::read_to_string(".txt") // reads file contents into a str
        .expect("Failed to read the file");


    println!("The total output is: {}", total_sum);
}