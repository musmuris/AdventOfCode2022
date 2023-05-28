
use std::collections::HashSet;

pub fn day3() {

    let input = include_str!("day3.txt");

    let mut acc : u32= 0;
    for line in input.lines() {
        let (first,second) = line.split_at(line.len()/2);
    
        let chars1 : HashSet<u8> = first.bytes().collect();
        let chars2 : HashSet<u8> = second.bytes().collect();
        let same = *(chars1.intersection(&chars2).next().unwrap());

        acc = acc + match same {
            b'a'..=b'z' => same - b'a' + 1,
            b'A'..=b'Z' => same - b'A' + 27,
            _ => panic!("something wonderful has happened")
        } as u32;
        
        println!("{}", acc);
    }

    println!("{}", acc);
}