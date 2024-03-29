const INPUT: &str = include_str!("inputs/day8.txt");

struct TreeChecker<'a> {
    trees: &'a Vec<&'a [u8]>,
}

impl<'a> TreeChecker<'a> {
    fn new(trees: &'a Vec<&'a [u8]>) -> TreeChecker<'a> {
        TreeChecker { trees }
    }

    fn check_tree_in_direction(
        &mut self,
        row: usize,
        col: usize,
        delta_row: isize,
        delta_col: isize,
    ) -> (bool, u32) {
        let mut visible = true;
        let mut score = 0;

        let current = self.trees[row][col];
        let mut row = (row as isize) + delta_row;
        let mut col = (col as isize) + delta_col;

        while let Some(&height) = self
            .trees
            .get(row as usize)
            .and_then(|x| x.get(col as usize))
        {
            score += 1;
            if current <= height {
                visible = false;
                break;
            }
            row += delta_row;
            col += delta_col;
        }

        (visible, score)
    }
}

pub fn day8(input: &str) -> (u32, u32) {
    let trees: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();

    let mut tc = TreeChecker::new(&trees);

    let (mut vis_count, mut max_score) = (0, 0);
    let rowlen = trees[0].len();

    for row in 0..trees.len() {
        for col in 0..rowlen {
            let (mut visible, mut score) = (false, 1);

            for (dr, dc) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                let (vis, sc) = tc.check_tree_in_direction(row, col, dr, dc);
                visible |= vis;
                score *= sc;
            }

            if visible {
                vis_count += 1;
            }
            if score > max_score {
                max_score = score;
            }
        }
    }

    (vis_count, max_score)
}

fn main() {
    let (p1, p2) = day8(INPUT);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("inputs/day8.test1.txt");
        let (p1, p2) = day8(input);

        assert_eq!(p1, 21);
        assert_eq!(p2, 8);
    }

    #[test]
    fn test_main() {
        let (p1, p2) = day8(INPUT);

        assert_eq!(p1, 1560);
        assert_eq!(p2, 252000);
    }
}
