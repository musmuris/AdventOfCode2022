use std::fs;
use std::cmp;

pub fn day1() {

    let file_path = r"C:\dev\AdventOfCode2022\src\input.txt";

    println!("Using file {}", file_path);

    let contents = fs::read_to_string(file_path)
         .expect("Should have been able to read the file");

    let mut acc = 0;
    let mut highest = 0;
    for line in contents.lines() {
        match line.parse::<i32>() {
            Ok(i) => acc += i,
            Err(_) => {
                highest = cmp::max(highest, acc);
                acc = 0;
            }
        }
    }
    println!("{}", highest  )
}