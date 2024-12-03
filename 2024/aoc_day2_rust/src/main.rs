use core::num;

#[derive(Debug, Clone, Copy)]
enum FileChoice {
    Test,
    Run,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    INC,
    DEC,
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
        FileChoice::Test => include_str!("../data/test.txt"),
        FileChoice::Run => include_str!("../data/list.txt")
    };

    let levels = parse_data(datafile);
    
    // compute safe (PART1)
    let mut safe_reports = 0;
    
    for level in levels {
        let is_inc = level
            .windows(2)
            .all(|le| {
                (le[1] > le[0]) && 
                (le[1] - le[0] >= 1) && 
                (le[1] - le[0] <= 3)
            });

        let is_dec = level
            .windows(2)
            .all(|le| {
                (le[1] < le[0]) && 
                (le[1] - le[0] <= -1) && 
                (le[1] - le[0] >= -3)
            });
        
        if is_inc || is_dec {
            safe_reports += 1;
        }
    }
    
    println!("sare reports: {}", safe_reports);
    
    // println!("distance: {}", distance);

    // // compute similarity (PART2)
    // let similarity: i32 = left.iter()
    //     .map(|l| {
    //         l * (right.iter().filter(|r| l == *r).count() as i32)
    //     }).sum();
        
    // println!("similarity: {}", similarity);
    
}

fn main() {
    println!("Test:");
    run(FileChoice::Test);

    println!("\nRun:");
    run(FileChoice::Run);

    // let start_time = std::time::Instant::now();
    // let duration = start_time.elapsed();
    // println!("\ntime taken duration_run: {:?}", duration);
    
}