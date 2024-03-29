use std::collections::HashMap;

const INPUT: &str = include_str!("inputs/day7.txt");

pub fn day7(input: &str) -> (u32, u32) {
    let mut current_dir = "root".to_string();
    let mut dir_stack: Vec<String> = Vec::new();
    let mut dirs: HashMap<String, u32> = HashMap::new();

    for line in input.lines() {
        if line.starts_with("$ cd") {
            let (_, name) = line.split_at(5);
            match name {
                ".." => {
                    cd_up(&mut dirs, &mut current_dir, &mut dir_stack);
                }
                "/" => {
                    cd_root(&mut dirs, &mut current_dir, &mut dir_stack);
                }
                rest => {
                    dir_stack.push(current_dir.clone());
                    current_dir.push('/');
                    current_dir.push_str(rest);
                }
            }
        } else if !line.starts_with("$ ls") && !line.starts_with("dir") {
            // Line is a size and filename
            let (size_str, _) = line.split_once(' ').unwrap();
            let size = size_str.parse::<u32>().unwrap();
            update_dir_size(&mut dirs, &current_dir, size);
        }
    }

    cd_root(&mut dirs, &mut current_dir, &mut dir_stack);

    let ans1 = dirs.values().cloned().filter(|&v| v <= 100000).sum();

    const FS_SIZE: u32 = 70000000;
    const NEED_SIZE: u32 = 30000000;

    let free_size = FS_SIZE - dirs.get("root").unwrap();
    let find_size = NEED_SIZE - free_size;
    let ans2 = dirs
        .values()
        .cloned()
        .filter(|&v| v >= find_size)
        .min()
        .unwrap();

    (ans1, ans2)
}

fn cd_up(dirs: &mut HashMap<String, u32>, current_dir: &mut String, dir_stack: &mut Vec<String>) {
    let size = dirs.get(current_dir.as_str()).cloned().unwrap_or(0);
    *current_dir = dir_stack.pop().unwrap();
    update_dir_size(dirs, current_dir.as_str(), size);
}

fn cd_root(dirs: &mut HashMap<String, u32>, current_dir: &mut String, dir_stack: &mut Vec<String>) {
    while !dir_stack.is_empty() {
        cd_up(dirs, current_dir, dir_stack)
    }
}

fn update_dir_size(dirs: &mut HashMap<String, u32>, dir_to_update: &str, size: u32) {
    dirs.get_mut(dir_to_update)
        .map(|s| {
            *s += size;
        })
        .unwrap_or_else(|| {
            dirs.insert(dir_to_update.to_string(), size);
        });
}
fn main() {
    let (p1, p2) = day7(INPUT);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("inputs/day7.test1.txt");
        let (p1, p2) = day7(input);

        assert_eq!(p1, 95437);
        assert_eq!(p2, 24933642);
    }

    #[test]
    fn test_main() {
        let (p1, p2) = day7(INPUT);

        assert_eq!(p1, 1390824);
        assert_eq!(p2, 7490863);
    }
}
