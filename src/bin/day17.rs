const INPUT: &str = include_str!("day17.txt");

struct Point {
    x: i64,
    y: i64,
}

type Rock = Vec<Point>;

fn build_rocks() -> Vec<Rock> {
    vec![
        vec![
            // #####
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 3, y: 0 },
        ],
        vec![
            // .#.
            Point { x: 0, y: 1 }, // ###
            Point { x: 1, y: 1 }, // .#.
            Point { x: 2, y: 1 },
            Point { x: 1, y: 0 },
            Point { x: 1, y: 2 },
        ],
        vec![
            // ..#
            Point { x: 0, y: 0 }, // ..#
            Point { x: 1, y: 0 }, // ###
            Point { x: 2, y: 0 },
            Point { x: 2, y: 1 },
            Point { x: 2, y: 2 },
        ],
        vec![
            // #
            Point { x: 0, y: 0 }, // #
            Point { x: 0, y: 1 }, // #
            Point { x: 0, y: 2 }, // #
            Point { x: 0, y: 3 },
        ],
        vec![
            // ##
            Point { x: 0, y: 0 }, // ##
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 0, y: 1 },
        ],
    ]
}

type Row = [bool; 7];

fn is_illegal_position(chamber: &[Row], rock: &Rock, pos: &Point) -> bool {
    // If the rock now is off the edge or hitting another rock then
    // it's in an illegal place
    rock
        .iter()
        .map(|Point { x, y }| (x + pos.x, y + pos.y))
        .any(|(x, y)| x < 0 || x > 6 || y < 0 || chamber[y as usize][x as usize])
}

fn lock_rock(chamber: &mut [Row], rock: &[Point], pos: &Point) -> i64 {
    let mut max_height = 0;
    for point in rock {
        let (x, y) = (point.x + pos.x, point.y + pos.y);

        chamber[y as usize][x as usize] = true;

        max_height = max_height.max(y);
    }

    max_height
}

// Set to true to see the visualiztion
static DO_DRAW : bool = false;

fn draw_chamber(label: &str, chamber: &[Row]) {
    if DO_DRAW {
        println!("{}", label);
        for row in chamber.iter().rev() {
            let row_str = row
                .iter()
                .map(|&x| if x { '#' } else { '.' })
                .collect::<String>();
            println!("{}", row_str);
        }
        println!("-------\n");
    }
}

fn dbg_draw_chamber(label: &str, chamber: &Vec<Row>, rock: &[Point], pos: &Point) {
    if DO_DRAW {
        let mut dbgchamber = chamber.clone();

        lock_rock(&mut dbgchamber, rock, &pos);

        draw_chamber(label, &dbgchamber)
    }
}

pub fn day17(input: &str) -> (usize, usize) {
    let rocks = build_rocks();
    let jets = input.trim().as_bytes();
    let mut jet_inx = 0usize;

    let mut chamber: Vec<Row> = Vec::new();
    let mut height_reached: i64 = 0;

    for rock_inx in 0..2022 {
        // Get next rock to fall
        let rock = &rocks[rock_inx % rocks.len()];

        // First check we have ennough space
        let zero_line = height_reached + 3;
        let need_height = zero_line as usize + 4;
        chamber.resize(need_height, [false; 7]);

        let mut rock_pos = Point { x: 2, y: zero_line };
                
        dbg_draw_chamber("start", &chamber, rock, &rock_pos);
        loop {
            let jet = match &jets[jet_inx % jets.len()] {
                b'<' => -1,
                b'>' => 1,
                _ => { continue; }
            };
            jet_inx += 1;
            let new_rock_pos = Point {
                x: rock_pos.x + jet,
                y: rock_pos.y,
            };
            if !is_illegal_position(&chamber, rock, &new_rock_pos) {
                rock_pos = new_rock_pos;
            }

            let l = format!("Move {}", jet);
            dbg_draw_chamber(&l, &chamber, rock, &rock_pos);

            // Now drop it one
            let new_rock_pos = Point {
                x: rock_pos.x,
                y: rock_pos.y - 1,
            };

            if is_illegal_position(&chamber, rock, &new_rock_pos) {
                // This time it's now locked.
                let rock_height = lock_rock(&mut chamber, rock, &rock_pos);
                height_reached = height_reached.max(rock_height + 1);
                break;
            }

            rock_pos = new_rock_pos;

            dbg_draw_chamber("Move down", &chamber, rock, &rock_pos);
        }
        draw_chamber(&format!("Locked {}", rock_inx), &chamber);        
    }    

    (height_reached as usize, input.len())
}

fn main() {
    let (p1, p2) = day17(INPUT);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("day17.test1.txt");
        let (p1, p2) = day17(input);

        assert_eq!(p1, 3068);
        assert_eq!(p2, input.len());
    }

    #[test]
    fn test_main() {
        let (p1, p2) = day17(INPUT);

        assert_eq!(p1, 3179);
        assert_eq!(p2, INPUT.len());
    }
}
