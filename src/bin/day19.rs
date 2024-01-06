use itertools::Itertools;

const INPUT: &str = include_str!("inputs/day19.txt");

struct BluePrint {
    id: u32,
    ore_need: u32,
    clay_need: u32,
    obsidian_need: (u32, u32),
    geode_need: (u32, u32),
}

#[derive(Default)]
struct State {
    minute: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,

    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
}

fn runBluePrint(b: &BluePrint) -> u32 {
    let initial = State {
        ore_robots: 1,
        ..State::default()
    };
    let mut stack = Vec::new();
    stack.push(initial);

    let mut max = 0;
    let max_ore_robots = b.clay_need.max(b.obsidian_need.0).max(b.geode_need.0);

    while let Some(state) = stack.pop() {
        let current = State {
            minute: state.minute + 1,
            ore: state.ore + state.ore_robots,
            clay: state.clay + state.clay_robots,
            obsidian: state.obsidian + state.obsidian_robots,
            geodes: state.geodes + state.geode_robots,
            ..state
        };
        if current.minute == 24 {
            max = max.max(current.geodes);
            continue;
        }
        let time_left = 24 - current.minute;
        if time_left * current.geode_robots + current.geodes + (time_left / 2)  < max {
          // println!("bail");
            //continue;
        }

        if state.ore >= b.geode_need.0 && state.obsidian >= b.geode_need.1 {
            stack.push(State {
                ore: current.ore - b.geode_need.0,
                obsidian: current.obsidian - b.geode_need.1,
                geode_robots: current.geode_robots + 1,
                ..current
            });
            continue;
        }
        if state.ore >= b.obsidian_need.0 && state.clay >= b.obsidian_need.1 && state.obsidian_robots < b.geode_need.1 && state.obsidian < b.geode_need.1 {
            stack.push(State {
                ore: current.ore - b.obsidian_need.0,
                clay: current.clay - b.obsidian_need.1,
                obsidian_robots: current.obsidian_robots + 1,
                ..current
            });
        }
        if state.ore >= b.clay_need && state.clay_robots < b.obsidian_need.1 && state.clay < b.obsidian_need.1 {
            stack.push(State {
                ore: current.ore - b.clay_need,
                clay_robots: current.clay_robots + 1,
                ..current
            });
        }
        if state.ore >= b.ore_need && state.ore_robots < max_ore_robots {
            stack.push(State {
                ore: current.ore - b.ore_need,
                ore_robots: current.ore_robots + 1,
                ..current
            });
        }

        stack.push(current);
    }

    println!("BP {} max is {}", b.id, max);

    return max;
}

pub fn day19(input: &str) -> (usize, usize) {
    let bluePrints = input
        .lines()
        .map(|l| {
            l.split(|c: char| !c.is_numeric())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u32>().unwrap())
                .collect_vec()
        })
        .map(|v| BluePrint {
            id: v[0],
            ore_need: v[1],
            clay_need: v[2],
            obsidian_need: (v[3], v[4]),
            geode_need: (v[5], v[6]),
        })
        .collect::<Vec<_>>();

    let mut q = 0;
    for b in bluePrints {
        q += b.id * runBluePrint(&b);
    }

    (q as usize, input.len())
}

fn main() {
    let (p1, p2) = day19(INPUT);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("inputs/day19.test1.txt");
        let (p1, p2) = day19(input);

        assert_eq!(p1, input.len());
        assert_eq!(p2, input.len());
    }

    #[test]
    fn test_main() {
        let (p1, p2) = day19(INPUT);

        assert_eq!(p1, INPUT.len());
        assert_eq!(p2, INPUT.len());
    }
}
