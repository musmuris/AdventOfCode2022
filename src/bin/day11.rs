use std::collections::VecDeque;

use nom::*;

const INPUT: &str = include_str!("inputs/day11.txt");

#[derive(Clone, Debug)]
enum Value {
    Old,
    Num(u64),
}

#[derive(Clone, Debug)]
enum Operation {
    Add(Value),
    Mul(Value),
}

enum WorryFn {
    Div3,
    ModMagic(u64),
}

#[derive(Clone, Debug)]
struct Monkey {
    id: u64,
    items: VecDeque<u64>,
    op: Operation,
    div: u64,
    true_monkey: u64,
    false_monkey: u64,
    check_count: u64,
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    // Consume all space
    let (input, _) = character::complete::multispace1(input)?;

    let (input, (op, _, val)) = sequence::preceded(
        bytes::complete::tag("Operation: new = old "),
        sequence::tuple((
            nom::branch::alt((
                nom::bytes::complete::tag("*"),
                nom::bytes::complete::tag("+"),
            )),
            nom::character::complete::space0,
            nom::branch::alt((
                nom::combinator::map(nom::character::complete::u64, Value::Num),
                nom::combinator::map(nom::bytes::complete::tag("old"), |_| Value::Old),
            )),
        )),
    )(input)?;

    let op = match op {
        "*" => Operation::Mul(val),
        "+" => Operation::Add(val),
        _ => panic!("unknown op"),
    };

    Ok((input, op))
}

fn parse_items(input: &str) -> IResult<&str, Vec<u64>> {
    // Consume all space
    let (input, _) = character::complete::multispace1(input)?;

    let (input, items) = sequence::preceded(
        bytes::complete::tag("Starting items: "),
        multi::separated_list1(bytes::complete::tag(", "), character::complete::u64),
    )(input)?;

    Ok((input, items))
}

// Monkey 0:
//   Starting items: 79, 98
//   Operation: new = old * 19
//   Test: divisible by 23
//     If true: throw to monkey 2
//     If false: throw to monkey 3
fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, monkey_id) = sequence::delimited(
        bytes::complete::tag("Monkey "),
        character::complete::u64,
        bytes::complete::tag(":"),
    )(input)?;

    let (input, items) = parse_items(input)?;
    let (input, op) = parse_operation(input)?;

    let (input, div) = nom::sequence::preceded(
        nom::sequence::pair(
            nom::character::complete::multispace0,
            nom::bytes::complete::tag("Test: divisible by "),
        ),
        nom::character::complete::u64,
    )(input)?;

    let (input, true_monkey) = nom::sequence::preceded(
        nom::sequence::pair(
            nom::character::complete::multispace0,
            nom::bytes::complete::tag("If true: throw to monkey "),
        ),
        nom::character::complete::u64,
    )(input)?;

    let (input, false_monkey) = nom::sequence::preceded(
        nom::sequence::pair(
            nom::character::complete::multispace0,
            nom::bytes::complete::tag("If false: throw to monkey "),
        ),
        nom::character::complete::u64,
    )(input)?;

    Ok((
        input,
        Monkey {
            id: monkey_id,
            items: items.into(),
            op,
            div,
            true_monkey,
            false_monkey,
            check_count: 0,
        },
    ))
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let (_, monkeys) = multi::separated_list1(
        sequence::pair(
            character::complete::line_ending,
            character::complete::line_ending,
        ),
        parse_monkey,
    )(input)
    .unwrap();

    monkeys
}

fn monkey_itemcheck(monkey: &mut Monkey, worry_fn: &WorryFn) -> Option<(u64, u64)> {
    let mut worry = monkey.items.pop_front().map(|i| match &monkey.op {
        Operation::Add(v) => match v {
            Value::Num(n) => i + n,
            Value::Old => i + i,
        },
        Operation::Mul(v) => match v {
            Value::Num(n) => i * n,
            Value::Old => i * i,
        },
    })?;

    monkey.check_count += 1;

    worry = match worry_fn {
        WorryFn::Div3 => worry / 3,
        WorryFn::ModMagic(magic) => worry % magic,
    };

    if worry % monkey.div == 0 {
        Some((worry, monkey.true_monkey))
    } else {
        Some((worry, monkey.false_monkey))
    }
}

fn run_turns(mut monkeys: Vec<Monkey>, turns: usize, worry_fn: &WorryFn) -> usize {
    for _ in 0..turns {
        for inx in 0..monkeys.len() {
            assert_eq!(inx as u64, monkeys[inx].id);

            while let Some((item, to_monkey)) =
                monkey_itemcheck(monkeys.get_mut(inx).unwrap(), worry_fn)
            {
                // println!(
                //     "Monkey {} throws item {} to monkey {}",
                //     inx, item, to_monkey
                // );
                let to_monkey = monkeys.get_mut(to_monkey as usize).unwrap();
                to_monkey.items.push_back(item);
            }
        }
    }

    monkeys.sort_by_key(|m| u64::MAX - m.check_count);

    (monkeys[0].check_count * monkeys[1].check_count) as usize
}

pub fn day11(input: &str) -> (usize, usize) {
    let monkeys = parse_input(input);

    // I had to look this up - it's "Chinese Remainer Theorum" and that fact that
    // you only need to know if the worry is divisible by each monkeys "div"
    // Once you know it I can see how it works and you could deduce this
    // and I wish I was clever ennough. And maybe I could have (though unlikely)
    // but I'm mainly here to learn rust
    let magic = monkeys.iter().fold(1, |acc, monkey| acc * monkey.div);

    let first = run_turns(monkeys.clone(), 20, &WorryFn::Div3);
    let second = run_turns(monkeys.clone(), 10000, &WorryFn::ModMagic(magic));

    (first, second)
}

fn main() {
    let (p1, p2) = day11(INPUT);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("inputs/day11.test1.txt");
        let (p1, p2) = day11(input);

        assert_eq!(p1, 10605);
        assert_eq!(p2, 2713310158);
    }

    #[test]
    fn test_main() {
        let (p1, p2) = day11(INPUT);

        assert_eq!(p1, 66124);
        assert_eq!(p2, 19309892877);
    }
}
