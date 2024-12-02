#[derive(Debug, Clone, Copy)]
enum FileChoice {
    Test,
    Run,
}

fn parse_data(puzzle: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = vec![];
    let mut right = vec![];
    
    for line in puzzle.lines() {
        let mut items = line.split_whitespace();

        left.push(items.next().unwrap().parse().unwrap());
        right.push(items.next().unwrap().parse().unwrap());
    }
    
    (left, right)
}

fn run(choice: FileChoice) {
    let datafile = match choice {
        FileChoice::Test => include_str!("../data/test.txt"),
        FileChoice::Run => include_str!("../data/list.txt")
    };

    let (mut left, mut right) = parse_data(datafile);
    
    // sorting arrays
    left.sort();
    right.sort();
    
    // compute distance (PART1)
    let distance: i32 = std::iter::zip(&left, &right)
        .map(|(l, r)| (l-r).abs() )
        .sum();
    
    println!("distance: {}", distance);

    // compute similarity (PART2)
    let similarity: i32 = left.iter()
        .map(|l| {
            l * (right.iter().filter(|r| l == *r).count() as i32)
        }).sum();
        
    println!("similarity: {}", similarity);
    
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