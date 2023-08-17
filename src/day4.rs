

struct Range {
    start: u32,
    end: u32
}

impl From<&str> for Range {
    fn from(s: &str) -> Self {
        let nums = s.split('-')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        Range {
            start: nums[0],
            end: nums[1]
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

pub fn day4() {
    
    let _input = 
"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    let input = include_str!("day4.txt");

    let lines = input.lines().collect::<Vec<_>>();

    let mut acc = 0;
    let mut acc2 = 0;
    for line in lines{
        let ranges = line.split(',')
            .map(|s| Range::from(s))
            .collect::<Vec<_>>();
        if ranges[0].contains(&ranges[1]) || ranges[1].contains(&ranges[0]){
            acc += 1;
        }
        if ranges[0].has_overlap(&ranges[1]) {
            acc2 += 1;
        }
    }
    println!("{}", acc);
    println!("{}", acc2);
}