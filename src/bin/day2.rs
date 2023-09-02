use std::collections::HashMap;

pub fn day2(input: &str) -> (i32, i32) {
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

    let mut total1 = 0;
    for line in input.lines() {
        match scores.get(line) {
            Some(&score) => total1 += score,
            _ => (),
        }
    }

    let scores2 = HashMap::from([
        ("A X", 3 + 0),
        ("A Y", 1 + 3),
        ("A Z", 2 + 6),
        ("B X", 1 + 0),
        ("B Y", 2 + 3),
        ("B Z", 3 + 6),
        ("C X", 2 + 0),
        ("C Y", 3 + 3),
        ("C Z", 1 + 6),
    ]);

    let mut total2 = 0;
    for line in input.lines() {
        match scores2.get(line) {
            Some(&score) => total2 += score,
            _ => (),
        }
    }

    (total1, total2)
}

fn main() {
    let input = include_str!("day2.txt");
    let (p1, p2) = day2(input);
    println!("{}\n{}", p1, p2);
}
