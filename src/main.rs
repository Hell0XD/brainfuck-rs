use std::env::args;
use std::fs::read_to_string;
use std::io::Read;
fn main() {
    let program: Vec<char> = read_to_string(args().skip(1).next().expect("Expected path to file"))
        .expect("Cannot read the file")
        .chars()
        .collect();
    let (mut mem, mut ptr, mut pc): (Vec<u8>, usize, usize) = (vec![0; 30_000], 0, 0);
    while let Some(char) = program.get(pc) {
        match char {
            '>' => ptr += 1,
            '<' => ptr -= 1,
            '+' => mem[ptr] += 1,
            '-' => mem[ptr] -= 1,
            '.' => print!("{}", mem[ptr] as char),
            ',' => mem[ptr] = std::io::stdin().bytes().next().unwrap().unwrap(),
            '[' if mem[ptr] == 0 => {
                let mut scopec: usize = 0;
                while !(program[pc] == ']' && scopec == 0) {
                    pc += 1;
                    match program[pc] {
                        '[' => scopec += 1,
                        ']' => scopec -= 1,
                        _ => (),
                    }
                }
            }
            ']' if mem[ptr] != 0 => {
                let mut scopec = 0;
                loop {
                    pc -= 1;
                    match program[pc] {
                        ']' => scopec += 1,
                        '[' if scopec == 0 => break,
                        '[' => scopec -= 1,
                        _ => (),
                    }
                }
            }
            _ => (),
        }
        pc += 1;
    }
}
