const INPUT: &str = include_str!("inputs/day4.txt");

struct Range {
    start: u32,
    end: u32,
}

impl From<&str> for Range {
    fn from(s: &str) -> Self {
        let nums = s
            .split('-')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        Range {
            start: nums[0],
            end: nums[1],
        }
    }
}

impl Range {
    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn has_overlap(&self, other: &Self) -> bool {
        self.start <= other.end && other.start <= self.end
    }
}

pub fn day4(input: &str) -> (i32, i32) {
    let lines = input.lines().collect::<Vec<_>>();

    let mut acc1 = 0;
    let mut acc2 = 0;
    for line in lines {
        let ranges = line.split(',').map(Range::from).collect::<Vec<_>>();
        if ranges[0].contains(&ranges[1]) || ranges[1].contains(&ranges[0]) {
            acc1 += 1;
        }
        if ranges[0].has_overlap(&ranges[1]) {
            acc2 += 1;
        }
    }
    (acc1, acc2)
}

fn main() {
    let (p1, p2) = day4(INPUT);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("inputs/day4.test1.txt");

        let (p1, p2) = day4(input);

        assert_eq!(p1, 2);
        assert_eq!(p2, 4);
    }

    #[test]
    fn test_main() {
        let (p1, p2) = day4(INPUT);

        assert_eq!(p1, 567);
        assert_eq!(p2, 907);
    }
}
