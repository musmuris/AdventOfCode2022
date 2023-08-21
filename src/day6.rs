use  std::collections::VecDeque;
use  std::collections::BTreeSet;

fn find_marker(input: &str, window_size: usize) -> Option<usize> {
    let mut seen: VecDeque<char> = VecDeque::new();
    for (inx, c) in input.chars().enumerate() {
        seen.push_back(c);
        if seen.len() > window_size {
            seen.pop_front();
        } 
        let dupes = seen.iter().collect::<BTreeSet<&char>>();
        if dupes.len() == window_size {
            return Some(inx+1);
        }
    }
    None
}

pub fn day6() {
    let input = include_str!("day6.txt");

    println!("test {:?}", find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb",4));
    println!("test {:?}", find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz",4));
    println!("test {:?}", find_marker("nppdvjthqldpwncqszvftbrmjlhg",4));
    println!("test {:?}", find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",4));
    println!("test {:?}", find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",4));
    
    println!("{}", find_marker(input, 4).unwrap()); 
    println!("{}", find_marker(input, 14).unwrap()); 

}
