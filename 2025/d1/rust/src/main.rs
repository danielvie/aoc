use std::fs;
use std::io;

enum Puzzle {
    Test,
    Puzzle1,
    Puzzle2,
}

fn run_password_1(txt: &String) {
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
                return;
            },
        };
        
        // parsing rest of the string
        let txt_rest = &li[1..];
        let num = match txt_rest.trim().parse::<i32>() {
            Ok(number) => number,
            Err(e) => {
                println!("something is not right!\ntxt_rest: {:?}\ne: {}", txt_rest, e.to_string());
                return
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
}

fn run_password_2(txt: &String) {
    let lines = txt.split("\r\n");
    
    let mut value  = 50;
    let mut value_ = 50;
    let mut count  = 0;

    println!("value  : {}", value);
    println!("count  : {}\n", count);

    for li in lines {
        // get direction
        let dir = match li.chars().next() {
            Some(c) => c,
            None => {
                println!("input is empty");
                return;
            },
        };
        
        // parsing rest of the string
        let txt_rest = &li[1..];
        let num = match txt_rest.trim().parse::<i32>() {
            Ok(number) => number,
            Err(e) => {
                println!("something is not right!\ntxt_rest: {:?}\ne: {}", txt_rest, e.to_string());
                return
            },
        };
        
        match dir {
            'R' => {
                for _ in 1..num {
                    value += 1;
                    if value > 99 {
                        value = 0;
                    }
                    
                    if value == 0 {
                        count += 1;
                    }
                }
            },
            'L' => {
                for _ in 1..num {
                    value -= 1;
                    if value < 0 {
                        value = 99;
                    }
                    
                    if value == 0 {
                        count += 1;
                    }
                }
            },
            _ => {}
        }
        

        value_ = value;
        
        // print results
        println!("dial is rotated {} to point at {}", li, value);
        println!("value  : {}", value);
        println!("value_ : {}", value_);
        println!("count  : {}\n", count);
    }
}


fn main() {
    match file_read(&Puzzle::Puzzle1) {
        Ok(txt) => {
            // run_password_1(&txt);
            run_password_2(&txt);
        }
        Err(e) => {
            eprintln!("niet ok\n{}", e.to_string());
        }
    }
}

fn file_read(puzzle: &Puzzle) -> Result<String, io::Error> {
    // let path = "data/test.txt";

    let file = match puzzle {
        Puzzle::Test => "data/test.txt",
        Puzzle::Puzzle1 => "data/puzzle1.txt",
        Puzzle::Puzzle2 => "data/puzzle2.txt",
    };

    let txt = fs::read_to_string(file)?;
    Ok(txt)
}

