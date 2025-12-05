use std::fs;
use std::io;
use std::error::Error;

// --- Enums and Structs ---

#[derive(Debug)]
enum Puzzle {
    Test,
    Puzzle,
}

#[derive(Debug)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug)]
struct Rotation {
    turn: Turn,
    distance: i32,
}

// reads file content
fn file_read(puzzle: &Puzzle) -> Result<String, io::Error> {
    let file = match puzzle {
        Puzzle::Test => "data/test.txt",
        Puzzle::Puzzle => "data/puzzle.txt",
    };

    fs::read_to_string(file)
}

// parses line into a rotation struct
fn parse_line(line: &str) -> Result<Rotation, String> {
    let line = line.trim();
    if line.is_empty() {
        return Err("Empty line".to_string());
    }

    let (dir_char, num_str) = line.split_at(1);
    
    let turn = match dir_char {
        "L" => Turn::Left,
        "R" => Turn::Right,
        _ => return Err(format!("Invalid direction: {}", dir_char)),
    };
    
    let distance = num_str.trim().parse::<i32>().map_err(|e| format!("Invalid distance '{}': {}", num_str, e))?;

    Ok(Rotation { turn, distance })
}

// parses the entire file content into a vector of rotation instructions
fn parse_input(txt: &str) -> Result<Vec<Rotation>, String> {
    txt.split_terminator('\n')
       .filter(|s| !s.trim().is_empty())
       .map(|line| parse_line(line.trim_end_matches('\r')))
       .collect()
}


// part 1
fn run_password_1(rotations: &[Rotation]) -> i32 {

    let mut value = 50;
    let mut count = 0;
    let modulo: i32 = 100;

    for rotation in rotations {
        // Calculate the new value
        match rotation.turn {
            Turn::Right => value += rotation.distance,
            Turn::Left => value -= rotation.distance,
        }

        // apply modulo
        value = value % modulo;
        if value < 0 {
            value += modulo;
        }
        
        if value == 0 {
            count += 1;
        }
    }
    
    count
}

// part 2
fn run_password_2(rotations: &[Rotation]) -> i32 {

    let mut value  = 50;
    let mut count  = 0;

    for rotation in rotations {
        
        let v_calc = if value == 0 { 100 } else { value };

        match rotation.turn {
            Turn::Right => {
                let crossing = (v_calc + rotation.distance) / 100 - (v_calc / 100);
                count += crossing;
                
                value = (value + rotation.distance) % 100;
            },
            Turn::Left => {
                let dist_to_zero = v_calc;
                
                if rotation.distance >= dist_to_zero {
                    let remaining_dist = rotation.distance - dist_to_zero;
                    count += 1 + (remaining_dist / 100);
                }
                
                value = ((value - rotation.distance) % 100 + 100) % 100;
            },
        }
    }
    
    count
}

fn main() -> Result<(), Box<dyn Error>> {
    
    let txt = file_read(&Puzzle::Puzzle)?;
    let rotations = parse_input(&txt).map_err(Box::<dyn Error>::from)?;

    let result_1 = run_password_1(&rotations);
    println!("Part 1 Result (1043): {}", result_1);
    
    let result_2 = run_password_2(&rotations);
    println!("Part 2 Result (5963): {}", result_2);

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_rotations() -> Vec<Rotation> {
        let txt = file_read(&Puzzle::Test).unwrap();
        parse_input(&txt).unwrap()
    }
    
    fn get_puzzle_rotations() -> Vec<Rotation> {
        let txt = file_read(&Puzzle::Puzzle).unwrap();
        parse_input(&txt).unwrap()
    }


    #[test]
    fn part1_test() {
        assert_eq!(run_password_1(&get_test_rotations()), 3);
    }

    #[test]
    fn part2_test() {
        assert_eq!(run_password_2(&get_test_rotations()), 6);
    }

    #[test]
    fn part1() {
        assert_eq!(run_password_1(&get_puzzle_rotations()), 1043);
    }

    #[test]
    fn part2() {
        assert_eq!(run_password_2(&get_puzzle_rotations()), 5963);
    }
}