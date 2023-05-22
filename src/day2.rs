
use std::collections::HashMap;

pub fn day2() {

    let input = include_str!("day2.txt");

    let scores = HashMap::from([
        ("A X", 1 + 3),
        ("A Y", 2 + 6),
        ("A Z", 3 + 0),
        ("B X", 1 + 0),
        ("B Y", 2 + 3),
        ("B Z", 3 + 6),
        ("C X", 1 + 6),
        ("C Y", 2 + 0),
        ("C Z", 3 + 3),
    ]);

    let mut total = 0;
    for line in input.lines() {
        match scores.get(line) {
            Some(&score) => total += score,
            _ => () 
        }
    }

    println!("{}", total)
}