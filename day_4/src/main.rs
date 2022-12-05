use std::fs;

pub mod lib;

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    
    println!("{:?}", lib::count_pairs(&contents));
    println!("{:?}", lib::count_pairs_2(&contents));

}
