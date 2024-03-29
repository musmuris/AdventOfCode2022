const INPUT: &str = include_str!("inputs/day8.txt");

pub fn day8(input: &str) -> (u32, u32) {
    let trees: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();

    let row_len = trees[0].len();
    let mut number = 0;
    let mut visible: Vec<bool> = vec![false; trees.len() * row_len];
    let mut max: u8;

    let mut check_tree = |row: usize, col: usize, max2: &mut u8| {
        if trees[row][col] > *max2 {
            *max2 = trees[row][col];
            if !visible[row * row_len + col] {
                visible[row * row_len + col] = true;
                number += 1;
            }
        }
    };

    for row in 0..trees.len() {
        max = 0;
        for col in 0..row_len {
            check_tree(row, col, &mut max);
        }
        max = 0;
        for col in (0..row_len).rev() {
            check_tree(row, col, &mut max);
        }
    }

    for col in 0..row_len {
        max = 0;
        for row in 0..trees.len() {
            check_tree(row, col, &mut max);
        }
        max = 0;
        for row in (0..trees.len()).rev() {
            check_tree(row, col, &mut max);
        }
    }

    (number, 0)
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
        let (p1, _p2) = day8(input);

        assert_eq!(p1, 21);
    }
}
