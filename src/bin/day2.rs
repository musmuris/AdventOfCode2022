use std::collections::HashMap;

const INPUT: &str = include_str!("inputs/day2.txt");

pub fn day2(input: &str) -> (i32, i32) {
    let scores = HashMap::from([
        ("A X", 1 + 3),
        ("A Y", 2 + 6),
        ("A Z", 3),
        ("B X", 1),
        ("B Y", 2 + 3),
        ("B Z", 3 + 6),
        ("C X", 1 + 6),
        ("C Y", 2),
        ("C Z", 3 + 3),
    ]);

    let mut total1 = 0;
    for line in input.lines() {
        if let Some(&score) = scores.get(line) {
            total1 += score;
        }
    }

    let scores2 = HashMap::from([
        ("A X", 3),
        ("A Y", 1 + 3),
        ("A Z", 2 + 6),
        ("B X", 1),
        ("B Y", 2 + 3),
        ("B Z", 3 + 6),
        ("C X", 2),
        ("C Y", 3 + 3),
        ("C Z", 1 + 6),
    ]);

    let mut total2 = 0;
    for line in input.lines() {
        if let Some(&score) = scores2.get(line) {
            total2 += score;
        }
    }

    (total1, total2)
}

fn main() {
    let (p1, p2) = day2(INPUT);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("inputs/day2.test1.txt");
        let (p1, p2) = day2(input);

        assert_eq!(p1, 15);
        assert_eq!(p2, 12);
    }

    #[test]
    fn test_main() {
        let (p1, p2) = day2(INPUT);

        assert_eq!(p1, 14827);
        assert_eq!(p2, 13889);
    }
}
