
use std::collections::HashSet;
use itertools::Itertools;

pub fn day3() {

    let input = include_str!("day3.txt");

    let mut acc : u32= 0;
    for line in input.lines() {

        if !line.is_ascii() {
            panic!("ASCII only string expected")
        }
        let (first,second) = line.split_at(line.len()/2);
    
        let chars1 : HashSet<u8> = first.bytes().collect();
        let chars2 : HashSet<u8> = second.bytes().collect();
        let sames = chars1.intersection(&chars2);
        let same = sames.at_most_one();

        acc = acc + map_priority(*same.unwrap().unwrap())        
    }

    println!("{}", acc);
    acc = 0;
    for set in &input.lines().chunks(3) {
        let chunk_lines : Vec<HashSet<u8>> = set.map(|x| x.bytes().collect()).collect();
        let mut found: Option<u8> = None;
        for c in &chunk_lines[0] {
            if chunk_lines[1].contains(c) && chunk_lines[2].contains(c) {
                found = match found {
                    None => Some(*c),
                    Some(_) => panic!("more than one found")
                }
            }
        }
        if found == None {
            panic!("No match found")
        }

        acc = acc + map_priority(found.unwrap())        
    }
    println!("{}", acc);
}

fn map_priority(item: u8) -> u32 {
    match item {
        b'a'..=b'z' => item - b'a' + 1,
        b'A'..=b'Z' => item - b'A' + 27,
        _ => panic!("Expected a..z or A..Z characters only")
    }.into()
}