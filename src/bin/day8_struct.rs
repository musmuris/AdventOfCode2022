struct TreeChecker<'a> {
    trees: &'a Vec<&'a [u8]>,
    row_len: usize,
    visible_count: usize,
    visible: Vec<bool>,
    max: u8,
}

impl<'a> TreeChecker<'a> {
    fn new(trees: &'a Vec<&'a [u8]>) -> TreeChecker<'a> {
        let row_len = trees[0].len();
        TreeChecker {
            trees,
            row_len,
            visible_count: 0,
            visible: vec![false; trees.len() * row_len],
            max: 0,
        }
    }

    fn check_tree(&mut self, row: usize, col: usize) {
        if self.trees[row][col] > self.max {
            self.max = self.trees[row][col];
            if !self.visible[row * self.row_len + col] {
                self.visible[row * self.row_len + col] = true;
                self.visible_count += 1;
            }
        }
    }

    fn check_row(&mut self, row: usize, col_range: impl Iterator<Item=usize>) {
        self.max = 0;
        for col in col_range {
            self.check_tree(row, col);
        }
    }

    fn check_col(&mut self, col: usize, row_range: impl Iterator<Item=usize>) {
        self.max = 0;
        for row in row_range {
            self.check_tree(row, col);
        }
    }

    fn check_trees(&mut self) -> usize {

        for row in 0..self.trees.len() {        
            self.check_row(row, 0..self.row_len);
            self.check_row(row, (0..self.row_len).rev());        
        }
    
        for col in 0..self.row_len {
            self.check_col(col, 0..self.trees.len());
            self.check_col(col, (0..self.trees.len()).rev());         
        }

        self.visible_count
    }
}

pub fn day8(input: &str) -> (usize, usize) {
    let trees: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();

    let mut tc = TreeChecker::new(&trees);

    let p1 = tc.check_trees();

    return (p1, 0);
}

fn main() {
    let input = include_str!("day8.txt");
    let (p1, p2) = day8(input);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let (p1, p2) = day8(
            "30373
25512
65332
33549
35390
",
        );

        assert_eq!(p1, 21);
        //assert_eq!(p2, 21);
    }
}
