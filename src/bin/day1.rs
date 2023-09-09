const INPUT: &str = include_str!("day1.txt");

pub fn day1(input: &str) -> (i32, i32) {
    let mut acc = 0;
    let mut elf = Vec::new();
    for line in input.lines() {
        match line.parse::<i32>() {
            Ok(i) => acc += i,
            Err(_) => {
                elf.push(acc);
                acc = 0;
            }
        }
    }
    elf.sort();
    elf.reverse();

    let part1 = elf[0];
    let part2 = elf.iter().take(3).sum::<i32>();

    (part1, part2)
}

fn main() {
    // Reading from file - left in as an example for
    // future Nigel when he forgets
    // let file_path = r"C:\dev\AdventOfCode2022\src\input.txt";

    // println!("Using file {}", file_path);

    // let contents = fs::read_to_string(file_path)
    //      .expect("Should have been able to read the file");

    let (p1, p2) = day1(INPUT);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("day1.test1.txt");
        let (p1, p2) = day1(input);

        assert_eq!(p1, 24000);
        assert_eq!(p2, 41000);
    }

    #[test]
    fn test_main() {
        let (p1, p2) = day1(INPUT);

        assert_eq!(p1, 72070);
        assert_eq!(p2, 211805);
    }
}
