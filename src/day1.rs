
pub fn day1() {

    let input = include_str!("day1_input.txt");

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
