use std::collections::{HashMap, VecDeque, BTreeSet};

use itertools::Itertools;
use nom::*;

const INPUT: &str = include_str!("day16.txt");

#[derive(Debug)]
struct Valve<'a> {
    name: &'a str,
    flow: u32,
    tunnels: Vec<&'a str>,
}

fn parse_valve(input: &str) -> IResult<&str, Valve> {
    let (input, name) = sequence::delimited(
        bytes::complete::tag("Valve "),
        character::complete::alpha1,
        bytes::complete::tag(" has flow rate"),
    )(input)?;

    let (input, flow) = nom::sequence::delimited(
        bytes::complete::tag("="),
        nom::character::complete::u32,
        nom::branch::alt((
            bytes::complete::tag("; tunnels lead to valves "),
            bytes::complete::tag("; tunnel leads to valve "),
        )),
    )(input)?;

    let (input, tunnels) =
        multi::separated_list1(bytes::complete::tag(", "), character::complete::alpha0)(input)?;

    Ok((
        input,
        Valve {
            name,
            flow,
            tunnels,
        },
    ))
}

fn parse_input(input: &str) -> HashMap<&str, Valve> {
    let (_, valves) =
        multi::separated_list1(character::complete::line_ending, parse_valve)(input).unwrap();

    valves
        .into_iter()
        .map(|x| (x.name, x))
        .collect::<HashMap<&str, Valve>>()
}

// END OF PARSING


// Find min distance from every node to every other 
// using Floyd–Warshal (https://en.wikipedia.org/wiki/Floyd–Warshall_algorithm)
fn find_distances<'a>(
    valves: &'a HashMap<&'a str, Valve<'a>>,
) -> HashMap<&'a str, HashMap<&'a str, u32>> {
    let mut ret = HashMap::new();

    // Set up Floyd–Warshal (https://en.wikipedia.org/wiki/Floyd–Warshall_algorithm)
    // Here all edges are 1 except loopbacks. All other routes start at u32::MAX (infinite)
    for (id, v) in valves.iter() {
        let mut dists = HashMap::new();
        for v2 in valves.keys() {
            dists.insert(
                *v2,
                if v.tunnels.contains(v2) {
                    1
                } else if v2 == id {
                    0
                } else {
                    u32::MAX / 2
                },
            );
        }
        ret.insert(*id, dists);
    }

    // Floyd–Warshal
    for k in valves.keys() {
        for i in valves.keys() {
            for j in valves.keys() {
                let new_dist = ret[i][k] + ret[k][j];

                if ret[i][j] > new_dist {
                    ret.get_mut(i).unwrap().insert(j, new_dist);
                }
            }
        }
    }

    // now remove all valves that have 0 flow, and loopbacks so we can
    // never get back to a 0 valve. Though we will start from one ('AA')
    for id in valves.keys() {
        ret.get_mut(id)
            .unwrap()
            .retain(|k, v| *v != 0 && valves[k].flow != 0);
    }

    ret
}

// State of each iteration
struct State<'a> {
    valves: &'a HashMap<&'a str, Valve<'a>>,
    dists: &'a HashMap<&'a str, HashMap<&'a str, u32>>,
    valves_on: BTreeSet<&'a str>,
    current_id: &'a str,
    flow_rate: u32,
    flow: u32,
    time_step: u32,
}

// Heavily borrowed from 
// https://github.com/NickyMeuleman/scrapyard/blob/main/advent_of_code/2022/solutions/src/day_16.rs
fn run_the_fun(
    valves: &HashMap<&str, Valve<'_>>,
    dists: &HashMap<&str, HashMap<&str, u32>>,
    max_time: u32,
) -> (u32,u32) {
    let openable_count = valves.iter().filter(|(_, v)| v.flow != 0).count();

    let mut q = VecDeque::new();

    q.push_back(State {
        valves,
        dists,
        valves_on: BTreeSet::new(),
        current_id: "AA",
        flow_rate: 0,
        flow: 0,
        time_step: 0,
    });

    let mut max_relived = 0u32;
    let mut max_relived_states : HashMap<BTreeSet<&str>, u32> = HashMap::new();

    while let Some(state) = q.pop_front() {

        let relieved = state.flow + state.flow_rate * (max_time - state.time_step);
        max_relived_states
            .entry(state.valves_on.clone())
            .and_modify(|v| *v = relieved.max(*v))
            .or_insert(relieved);

        if state.valves_on.len() == openable_count {            
            max_relived = max_relived.max(relieved);
            continue;
        }

        for (dest, cost) in &state.dists[state.current_id] {
            if state.valves_on.contains(dest) {
                continue;
            }

            // Do we have time to walk there and turn it on?
            let time_taken = cost + 1;
            if state.time_step + time_taken >= max_time {
                // no - let it flow!
                let relieved = state.flow + state.flow_rate * (max_time - state.time_step);
                max_relived = max_relived.max(relieved);
                continue;
            }

            let mut new_on = state.valves_on.clone();
            new_on.insert(dest);

            q.push_back(State {
                valves: state.valves,
                dists: state.dists,
                valves_on: new_on,
                current_id: dest,
                flow_rate: state.flow_rate + state.valves[dest].flow,
                flow: state.flow + state.flow_rate * time_taken,
                time_step: state.time_step + time_taken,
            });
        }
    }

    // And this bit just blatently copied from 
    // https://github.com/NickyMeuleman/scrapyard/blob/main/advent_of_code/2022/solutions/src/day_16.rs
    // But doing so as i'm here to learn the language and how it works..
    // This is also quiet clever IMHO.
    let twofer = max_relived_states
        .iter()
        .tuple_combinations()
        .filter(|(human, elephant)| human.0.is_disjoint(elephant.0))
        .map(|(human, elephant)| human.1 + elephant.1)
        .max()
        .unwrap();

    (max_relived, twofer)
}

pub fn day16(input: &str) -> (usize, usize) {
    let valves = parse_input(input);
    let dists = find_distances(&valves);
    let p1 = run_the_fun(&valves, &dists, 30).0;
    let p2 = run_the_fun(&valves, &dists, 26).1;

    (p1 as usize, p2 as usize)
}

fn main() {
    let (p1, p2) = day16(INPUT);

    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("day16.test1.txt");
        let (p1, p2) = day16(input);

        assert_eq!(p1, 1651);
        assert_eq!(p2, 1707);
    }

    #[test]
    fn test_main() {
        let (p1, p2) = day16(INPUT);

        assert_eq!(p1, 1720);
        assert_eq!(p2, 2582);
    }
}
