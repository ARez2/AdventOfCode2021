use std::fs;

enum Commands {
    UP,
    DOWN,
    FORWARD,
}


struct Submarine {
    horizontal_pos: isize,
    depth: isize,
    aim: isize,
}


fn main() {
    let mut sub = Submarine {
        horizontal_pos: 0,
        depth: 0,
        aim: 0,
    };

    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    
    let mut last_cmd = Commands::FORWARD;
    for line in contents.split_whitespace() {
        let mut line_is_count = false;
        match line {
            "up" => {last_cmd = Commands::UP},
            "down" => {last_cmd = Commands::DOWN},
            "forward" => {last_cmd = Commands::FORWARD},
            _ => {line_is_count = true},
        };
        if line_is_count {
            let mut count = 0;
            
            let trimmed = line.trim();
            let line_res = trimmed.parse::<isize>();
            match line_res {
                Ok(x) => count = x,
                Err(err) => eprintln!("Error while converting to int: {}", err),
            };
            
            match last_cmd {
                Commands::UP => sub.aim -= count,
                Commands::DOWN => sub.aim += count,
                Commands::FORWARD => {
                    sub.horizontal_pos += count;
                    sub.depth += sub.aim * count;
                },
            }
        }
    };

    println!("Depth: {}", sub.depth);
    println!("Pos: {}", sub.horizontal_pos);
    println!("Res: {}", sub.horizontal_pos * sub.depth);
}
