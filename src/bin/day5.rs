pub fn day5(input: &str) -> (String, String) {
    let delim = if input.contains("\r\n") {
        "\r\n\r\n"
    } else {
        "\n\n"
    };

    let (stack_lines, move_lines) = input.split_once(delim).unwrap();

    let mut stack_iterator = stack_lines.lines().rev();
    let first = stack_iterator.next().unwrap();

    let last_col = first.split_ascii_whitespace().last().unwrap();
    let stack_count = last_col.parse().unwrap();
    let mut stacks = vec![vec![]; stack_count];

    for line in stack_iterator {
        for (index, character) in line.chars().enumerate().skip(1).step_by(4) {
            if character.is_ascii_alphabetic() {
                let stack_num = (index - 1) / 4;
                stacks[stack_num].push(character);
            }
        }
    }

    let mut stacks2 = stacks.clone();
    for line in move_lines.lines() {
        let mut s = line.split_ascii_whitespace();
        let count: usize = s.by_ref().skip(1).next().unwrap().parse().unwrap();
        let from: usize = s.by_ref().skip(1).next().unwrap().parse().unwrap();
        let to: usize = s.skip(1).next().unwrap().parse().unwrap();

        // Part 1
        for _ in 0..count {
            let p = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(p)
        }

        // Why can't I  (works if you get len then split_off on two lines)
        // But I guess here the imutable borrow happens after the mutable one
        // let end = stacks2[from-1].split_off(stacks2[from-1].len() - count);
        // stacks2[to-1].extend(end);

        // Or
        // let end = stacks2[from-1].iter().rev().take(3);
        // stacks2[to-1].extend(end);

        let n = stacks2[from - 1].len() - count;
        let end = stacks2[from - 1].split_off(n);
        stacks2[to - 1].extend(end);
    }

    (make_result(&stacks), make_result(&stacks2))
}

fn make_result(stacks: &Vec<Vec<char>>) -> String {
    let mut result = String::new();
    for stack in stacks.iter() {
        result.push(stack[stack.len() - 1]);
    }
    result
}

fn main() {
    let input = include_str!("day5.txt");
    let (p1, p2) = day5(input);
    println!("{}\n{}", p1, p2);
}
