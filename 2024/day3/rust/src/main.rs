
#[derive(Debug, Clone, Copy)]
enum FileChoice {
    Test,
    // Test2,
    // Puzzle,
}

enum Instruction {
    Mul(u32, u32),
}

fn parse_data(puzzle: &str) -> Vec<Vec<i32>> {
    // vector to store sequences
    let mut levels: Vec<Vec<i32>> = Vec::new();
    
    // split the input
    for line in puzzle.lines() {

        let level: Vec<i32> = line
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();

        // only add non-empty
        if !level.is_empty() {
            levels.push(level);
        }
    }
    
    levels
}

fn run(choice: FileChoice) {
    let datafile = match choice {
        FileChoice::Test => include_str!("../../data/test.txt"),
        // FileChoice::Test2 => include_str!("../../data/test2.txt"),
        // FileChoice::Puzzle => include_str!("../../data/puzzle.txt")
    };

    let levels = parse_data(datafile);
    
    // compute safe (PART1)
    let safe_reports: u32 = levels.iter()
            .map(|l| is_safe(&l) as u32)
            .sum();

    println!("safe reports: {}", safe_reports);
    
    // compute safer (PART2)
    let safer_reports: u32 = levels.iter()
            .map(|l| is_safe_damped(&l) as u32)
            .sum();
    
    println!("safe damped reports: {}", safer_reports);
}

fn main() {
    println!("Test:");
    run(FileChoice::Test);

    // println!("\nPuzzle:");
    // let start_time = std::time::Instant::now();
    // run(FileChoice::Puzzle);
    // let duration = start_time.elapsed();

    // println!("\ntime taken duration_run: {:?}", duration);
}