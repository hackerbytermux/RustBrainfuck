//BrainFuck on Rust
fn user_input() -> u8 {
    let mut input = String::new();
    println!("> ");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let char = input.trim().chars().next().unwrap() as u8;
    char
}

fn brain_run(program: String) {
    let mut memory: Vec<u32> = vec![0; 1000];
    let mut ptr: usize = 0;

    let mut i: usize = 0;

    while i < program.len() {
        let cmd = program.chars().nth(i).unwrap();
        match cmd {
            '>' => ptr += 1,
            '<' => ptr -= 1,
            '+' => memory[ptr] += 1,
            '-' => memory[ptr] -= 1,
            '.' => print!("{}", char::from_u32(memory[ptr]).unwrap()),
            ',' => memory[ptr] = user_input() as u32,
            '[' => {
                if memory[ptr] == 0 {
                    // find matching ]
                    let mut count = 1;
                    while count > 0 {
                        i += 1;
                        if program.chars().nth(i).unwrap() == '[' {
                            count += 1;
                        } else if program.chars().nth(i).unwrap() == ']' {
                            count -= 1;
                        }
                    }
                }
            }
            ']' => {
                if memory[ptr] != 0 {
                    // find matching [
                    let mut count = 1;
                    while count > 0 {
                        i -= 1;
                        if program.chars().nth(i).unwrap() == ']' {
                            count += 1;
                        } else if program.chars().nth(i).unwrap() == '[' {
                            count -= 1;
                        }
                    }
                }
            }
            _ => (),
        }
        i += 1;
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    //read brainfuck file
    let mut contents = String::new();
    if args.len() > 1 {
        let filename = &args[1];
        contents = std::fs::read_to_string(filename).expect("Could not read file");
    } else {
        println!("Usage: {} <filename>", &args[0]);
    }

    let _ = brain_run(contents);
}
