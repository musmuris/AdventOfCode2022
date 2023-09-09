const INPUT: &str = include_str!("day0.txt");

pub fn day0(input: &str) -> (usize, usize) {
    (input.len(), input.len())
}

fn main() {
    let (p1, p2) = day0(INPUT);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("day0.test1.txt");
        let (p1, p2) = day0(input);

        assert_eq!(p1, 23);
        assert_eq!(p2, 23);
    }

    #[test]
    fn test_main() {
        let (p1, p2) = day0(INPUT);

        assert_eq!(p1, 23);
        assert_eq!(p2, 23);
    }
}
