const INPUT: &str = include_str!("day10.txt");


struct Cpu {
    x: i32,
    pc: i32,

    acc: i32,
    crt: String,
}

impl Cpu {
    fn tick(&mut self) {
        
        let pixel = self.pc % 40;
        self.crt.push( if pixel >= self.x - 1 && pixel <= self.x + 1 { '#' } else {'.'} );
        
        self.pc +=1;
        if (self.pc - 20) % 40 == 0 {
            self.acc += self.x * self.pc;            
        }
        if pixel == 39 { self.crt.push('\n') };
    }
}

pub fn day10(input: &str) -> (i32, String) {

    let mut cpu = Cpu{x:1, pc:0, acc:0, crt : String::new()};
    for line in input.lines() {
        cpu.tick();
        if line.starts_with("addx") {            
            let op = line[5..].parse::<i32>().unwrap();
            cpu.tick();
            cpu.x += op;            
        }
    }
    
    (cpu.acc, cpu.crt)
}

fn main() {
    let (p1, p2) = day10(INPUT);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("day10.test1.txt");
        let p2pass = include_str!("day10.test1.result.txt").replace("\r\n", "\n");
        let (p1, p2) = day10(input);
        
        assert_eq!(p1, 13140);
        assert_eq!(p2, p2pass);
    }

    #[test]
    fn test_main() {
        let (p1, p2) = day10(INPUT);
        let main_pass = include_str!("day10.main.result.txt").replace("\r\n", "\n");

        assert_eq!(p1, 14240);
        assert_eq!(p2, main_pass); // PLULKBZH
     }
}
