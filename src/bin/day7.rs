use std::collections::HashMap;

pub fn day7(input: &str) -> (u32, u32) {

    let mut current_dir = "root".to_string();
    let mut dir_stack: Vec<String> = Vec::new();
    let mut dirs: HashMap<String,u32> = HashMap::new();
        
    for line in input.lines() {        
        if line.starts_with("$ cd") {
            let(_,name) = line.split_at(5);
            match name {
                ".." => {                    
                    cd_up(&mut dirs, &mut current_dir, &mut dir_stack);                    
                },
                "/" => { 
                    cd_root(&mut dirs, &mut current_dir, &mut dir_stack);                    
                },
                rest => { 
                    dir_stack.push(current_dir.clone());
                    current_dir.push_str("/"); 
                    current_dir.push_str(rest);                     
                }
            }
        } else if !line.starts_with("$ ls") && !line.starts_with("dir") {
            // Line is a size and filename
            let (size_str, _) = line.split_once(" ").unwrap();
            let size = size_str.parse::<u32>().unwrap();            
            update_dir_size(&mut dirs, &current_dir, size);
        }
    }            

    cd_root(&mut dirs, &mut current_dir, &mut dir_stack);

    let ans1 = dirs.values().cloned()
        .filter(|&v| v <= 100000)        
        .fold(0, |a, v| a + v);

    const FS_SIZE: u32 = 70000000;
    const NEED_SIZE: u32 = 30000000;

    let free_size = FS_SIZE - dirs.get("root").unwrap();
    let find_size = NEED_SIZE - free_size;

    let ans2 = dirs.values().cloned()
        .filter(|&v| v >= find_size )        
        .min().unwrap();

    return (ans1, ans2);
}

fn cd_up(dirs: &mut HashMap<String, u32>, current_dir: &mut String, dir_stack: &mut Vec<String>) {
    let size = dirs.get(current_dir.as_str()).cloned().unwrap_or(0);
    *current_dir = dir_stack.pop().unwrap();
    update_dir_size(dirs, current_dir.as_str(), size);
}

fn cd_root(dirs: &mut HashMap<String, u32>, current_dir: &mut String, dir_stack: &mut Vec<String>) {
    while dir_stack.len() > 0 {
        cd_up(dirs, current_dir, dir_stack)
    }
}

fn update_dir_size(dirs: &mut HashMap<String, u32>, dir_to_update: &str, size: u32) {    
    dirs.get_mut(dir_to_update)
        .map(|s| { *s += size; } )
        .unwrap_or_else(|| {
            dirs.insert(dir_to_update.to_string(), size ); 
        });
}

fn main() {
    let input = include_str!("day7.txt");
    let (p1,p2) = day7(input);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1()
    {
        let (p1,p2) = day7("$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k");
        
        assert_eq!(p1, 95437);
        assert_eq!(p2, 24933642);        
    }
}