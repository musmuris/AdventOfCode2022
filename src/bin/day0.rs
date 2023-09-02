
pub fn day0(input: &str) -> (usize, usize) {

    return (input.len(), input.len());
}

fn main() {
    let input = include_str!("day0.txt");
    let (p1,p2) = day0(input);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1()
    {
        let (p1,p2) = day0("1234");
        
        assert_eq!(p1, 4);
        assert_eq!(p2, 4);        
    }
}