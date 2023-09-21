const INPUT: &str = include_str!("day12.txt");

pub fn day12(input: &str) -> (usize, usize) {
    (input.len(), input.len())
}

fn main() {
    let (p1, p2) = day12(INPUT);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("day12.test1.txt");
        let (p1, p2) = day12(input);

        assert_eq!(p1, 23);
        assert_eq!(p2, 23);
    }

    #[test]
    fn test_main() {
        let (p1, p2) = day12(INPUT);

        assert_eq!(p1, 23);
        assert_eq!(p2, 23);
    }
}
