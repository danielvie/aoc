
#[derive(Debug, Clone, Copy)]
enum FileChoice {
    Test,
    Test2,
    Puzzle,
}

fn process(s: &str) -> usize {
    let mut pos = 0;
    let mut total = 0;

    while let Some(loc) = &s[pos..].find("mul(") {
        pos += loc + 4;
        let Some(value) = multiply(&s[pos..]) else {
            continue;
        };
        total += value;
    }
    total
}

fn process_conditional<S: AsRef<str>>(s: S) -> usize {
    let s = s.as_ref();
    let mut pos = 0;
    let mut total = 0;
    let mut enabled = true;
    while let Some(loc) = get_next(&s[pos..]) {
        pos += loc;
        match &s[pos..pos+3] {
            "mul" => {
                if enabled {
                    if let Some(product) = multiply(&s[pos + 4..]) {
                        total += product;
                    }
                }
            }
            "do(" => enabled = true,
            "don" => enabled = false,
            _ => panic!("bad pattern: {}", &s[pos..pos + 3]),
        }
        pos += 4;
    }
    
    total
}

fn multiply(s: &str) -> Option<usize> {
    let (left, rest) = s.split_once(',')?;
    let left = left.parse::<usize>().ok()?;
    let (right, _) = rest.split_once(')')?;
    let right = right.parse::<usize>().ok()?;
    
    Some(left*right)
}

fn get_next(s: &str) -> Option<usize> {
    let mul_loc = s.find("mul(");
    let do_loc = s.find("do()");
    let dont_loc = s.find("don't()");
    
    [mul_loc, do_loc, dont_loc].iter()
        .filter_map(|loc| *loc)
        .min()
}

fn run(choice: FileChoice) {
    let puzzle = match choice {
        FileChoice::Test => include_str!("../../data/test.txt"),
        FileChoice::Test2 => include_str!("../../data/test2.txt"),
        FileChoice::Puzzle => include_str!("../../data/puzzle.txt")
    };
    
    // parse file 
    // parse_puzzle(&puzzle);

    // compute
    println!("{:?}", process(&puzzle));
    println!("{:?}", process_conditional(&puzzle));
    
}

fn main() {
    println!("Test:");
    run(FileChoice::Test);

    println!("\nTest2:");
    run(FileChoice::Test2);


    println!("\nPuzzle:");
    // let start_time = std::time::Instant::now();
    run(FileChoice::Puzzle);
    // let duration = start_time.elapsed();

    // println!("\ntime taken duration_run: {:?}", duration);
}