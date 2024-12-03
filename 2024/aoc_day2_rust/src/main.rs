
#[derive(Debug, Clone, Copy)]
enum FileChoice {
    Test,
    Run,
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

fn is_safe(level: &Vec<i32>) -> bool {

    if level.len() == 1 {
        return false;
    }

    let is_inc = level
        .windows(2)
        .all(|le| {
            (le[1] > le[0]) && 
            (1..=3).contains(&(le[1] - le[0]))
        });

    let is_dec = level
        .windows(2)
        .all(|le| {
            (le[1] < le[0]) && 
            (-3..=-1).contains(&(le[1] - le[0]))
        });
    
    return is_inc || is_dec;
}

fn is_safe_damped(level: &Vec<i32>) -> bool {

    if is_safe(level) {
        return true;
    }
    
    (0..level.len())
        .map(|i| {
            let mut level_ = level.clone();
            level_.remove(i);

            is_safe(&level_)
        })
        .any(|is_valid| is_valid)
}

fn run(choice: FileChoice) {
    let datafile = match choice {
        FileChoice::Test => include_str!("../data/test.txt"),
        FileChoice::Run => include_str!("../data/list.txt")
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

    println!("\nRun:");
    let start_time = std::time::Instant::now();
    run(FileChoice::Run);
    let duration = start_time.elapsed();

    println!("\ntime taken duration_run: {:?}", duration);
}