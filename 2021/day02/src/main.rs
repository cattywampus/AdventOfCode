use std::fs;

struct Position {
    aim: i32,
    depth: i32,
    horizontal: i32,
}

impl Position {
    fn navigate(&mut self, command: Option<&str>, value: i32) {
        match command {
            Some("forward") => {
                self.horizontal += value;
                self.depth += self.aim * value;
            },
            Some("down") => self.aim += value,
            Some("up") => self.aim -= value,
            _ => {}
        }
    }

    fn new() -> Self {
        Position {
            aim: 0,
            horizontal: 0,
            depth: 0
        }
    }
}

fn main() {
    let contents = fs::read_to_string("resources/input.txt").unwrap();

    let position = navigate(&*contents);

    println!("Final destination is...");
    println!("  horizontal: {}", position.horizontal);
    println!("  depth: {}", position.depth);
    println!("  position: {}", position.horizontal * position.depth);
}

fn navigate(instructions: &str) -> Position {
    let mut position = Position::new();
    for line in instructions.lines() {
        let mut iter = line.split_whitespace();
        let command = iter.next();
        let value: i32 = match iter.next() {
            Some(x) => x.parse().unwrap(),
            _ => { panic!("Unable to parse instruction value: {}", line) }
        };
        position.navigate(command, value);
    }
    position
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_position() {
        let instructions = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2";
        let position = navigate(instructions);
        assert_eq!(15, position.horizontal);
        assert_eq!(60, position.depth);
        assert_eq!(900, position.horizontal * position.depth);
    }
}