
#[derive(Debug, Clone, Copy)]
enum FileChoice {
    Test,
    // Test2,
    Puzzle,
}

fn parse_puzzle(puzzle: &str) {
    println!("{}", puzzle);
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

fn multiply(s: &str) -> Option<usize> {
    let (left, rest) = s.split_once(',')?;
    let left = left.parse::<usize>().ok()?;
    let (right, _) = rest.split_once(')')?;
    let right = right.parse::<usize>().ok()?;
    
    Some(left*right)
}

fn run(choice: FileChoice) {
    let puzzle = match choice {
        FileChoice::Test => include_str!("../../data/test.txt"),
        // FileChoice::Test2 => include_str!("../../data/test2.txt"),
        FileChoice::Puzzle => include_str!("../../data/puzzle.txt")
    };
    
    // parse file 
    // parse_puzzle(&puzzle);

    // compute
    println!("{:?}", process(&puzzle));
    
}

fn main() {
    println!("Test:");
    run(FileChoice::Test);

    println!("\nPuzzle:");
    // let start_time = std::time::Instant::now();
    run(FileChoice::Puzzle);
    // let duration = start_time.elapsed();

    // println!("\ntime taken duration_run: {:?}", duration);
}