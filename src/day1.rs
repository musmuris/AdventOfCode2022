
pub fn day1() {

    // Reading from file - left in as an example for
    // future Nigel when he forgets
    // let file_path = r"C:\dev\AdventOfCode2022\src\input.txt";

    // println!("Using file {}", file_path);

    // let contents = fs::read_to_string(file_path)
    //      .expect("Should have been able to read the file");

    let input = include_str!("day1.txt");

    let mut acc = 0;
    let mut elf = Vec::new();
    for line in input.lines() {
        match line.parse::<i32>() {
            Ok(i) => acc += i,
            Err(_) => {
                elf.push(acc);
                acc = 0;
            }
        }
    }   
    elf.sort();
    elf.reverse();

    println!("{}", elf[0] );
    println!("{}", elf.into_iter().take(3).sum::<i32>() );

}
