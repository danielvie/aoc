use std::fs;
use std::io;

enum Puzzle {
    Test,
    Puzzle,
}

enum Turn {
    Left,
    Right,
}

fn run_password_1(puzzle: &Puzzle) -> i32 {

    let txt = match file_read(&puzzle) {
        Ok(txt) => txt,
        Err(_e) => panic!(),
    };

    let lines = txt.split("\r\n");
    
    let mut value = 50;
    let mut count = 0;

    println!("value  : {}\n", value);

    for li in lines {
        // get direction
        //
        let dir = match li.chars().next() {
            Some(c) => c,
            None => {
                println!("input is empty");
                panic!()
            },
        };
        
        // parsing rest of the string
        let txt_rest = &li[1..];
        let num = match txt_rest.trim().parse::<i32>() {
            Ok(number) => number,
            Err(e) => {
                println!("something is not right!\ntxt_rest: {:?}\ne: {}", txt_rest, e.to_string());
                panic!()
            },
        };
        
        match dir {
            'R' => {
                value += num;
                while value > 99 {
                    value -= 100;
                }
            }
            'L' =>  {
                value -= num;
                while value < 0 {
                    value += 100;
                }

            }
            _ => {}
        }
        

        if value == 0 {
            count += 1;
        }
        
        println!("command: {}", li);
        println!("value  : {}", value);
        println!("count  : {}\n", count);
    }
    
    count
}

fn run_password_2(puzzle: &Puzzle) -> i32 {

    let txt = match file_read(&puzzle) {
        Ok(txt) => txt,
        Err(_e) => panic!(),
    };

    let lines = txt.split("\r\n");
    
    let mut value  = 50;
    let mut count  = 0;

    println!("value  : {}", value);
    println!("count  : {}\n", count);

    for li in lines {

        let instruction = li.split_at(1);

        // get direction
        let dir = match instruction.0 {
            "L" => Turn::Left,
            "R" => Turn::Right,
            _ => panic!(),
        };
        
        // parsing rest of the string
        let txt_rest = &li[1..];
        let num = match txt_rest.trim().parse::<i32>() {
            Ok(number) => number,
            Err(e) => {
                println!("something is not right!\ntxt_rest: {:?}\ne: {}", txt_rest, e.to_string());
                panic!()
            },
        };
        
        match dir {
            Turn::Right => {
                for _ in 0..num {
                    value += 1;
                    if value > 99 {
                        value = 0;
                    }
                    
                    if value == 0 {
                        count += 1;
                    }
                }
            },
            Turn::Left => {
                for _ in 0..num {
                    value -= 1;
                    if value < 0 {
                        value = 99;
                    }
                    
                    if value == 0 {
                        count += 1;
                    }
                }
            },
        }
        
        // print results
        println!("dial is rotated {} to point at {}", li, value);
        println!("value  : {}", value);
        println!("count  : {}\n", count);
    }
    
    count
}


fn main() {
    // run_password_1(&Puzzle::Test);
    run_password_2(&Puzzle::Puzzle);
}

fn file_read(puzzle: &Puzzle) -> Result<String, io::Error> {
    // let path = "data/test.txt";

    let file = match puzzle {
        Puzzle::Test => "data/test.txt",
        Puzzle::Puzzle => "data/puzzle.txt",
    };

    let txt = fs::read_to_string(file)?;
    Ok(txt)
}


#[cfg(test)]
mod tests {
    // This brings all items from the outer module (like the `add` function)
    // into the scope of the test module.
    use super::*;

    #[test]
    fn part1_test() {
        // Assertions are used to check for expected results.
        assert_eq!(run_password_1(&Puzzle::Test), 3);
    }

    #[test]
    fn part2_test() {
        // You can use `assert!` for boolean checks
        assert_eq!(run_password_2(&Puzzle::Test), 6);
    }

    #[test]
    fn part1() {
        // You can use `assert!` for boolean checks
        assert_eq!(run_password_1(&Puzzle::Puzzle), 1043);
    }

    #[test]
    fn part2() {
        // You can use `assert!` for boolean checks
        assert_eq!(run_password_2(&Puzzle::Puzzle), 5963);
    }

}