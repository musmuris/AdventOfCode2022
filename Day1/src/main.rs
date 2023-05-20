use std::env;
use std::fs;
use std::cmp;

fn main() {

    let args: Vec<String> = env::args().collect();

    let file_path;
    if args.len() < 2 {
        file_path = r"C:\dev\AdventOfCode2022\Day1\input.txt"
    } else {    
        file_path = &args[1];
    }

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