pub fn day5() {
    let input = include_str!("day5.txt");

    let (stack_lines, move_lines) = input.split_once("\r\n\r\n").unwrap();

    let mut s = stack_lines.lines().rev();
    let first = s.next().unwrap();

    let last_col = first.split_ascii_whitespace().last().unwrap();
    let stack_count = last_col.parse().unwrap();
    let mut stacks = vec![vec![]; stack_count];

    for line in s {
        for (index, character) in line.chars().enumerate().skip(1).step_by(4) {
            if character.is_ascii_alphabetic() {
                let stack_num = (index - 1) / 4;
                stacks[stack_num].push(character);
            }
        }
    }

    println!("{:?}", stacks);
    
}
