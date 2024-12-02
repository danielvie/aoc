#[derive(Debug, Clone, Copy)]
enum FileChoice {
    Test,
    Run,
}

fn parse_puzzle(puzzle: &str) -> (Vec<i32>, Vec<i32>) {
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
    let puzzle = match choice {
        FileChoice::Test => include_str!("../puzzle/test.txt"),
        FileChoice::Run => include_str!("../puzzle/list.txt")
    };

    let (mut left, mut right) = parse_puzzle(puzzle);
    
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
    run(FileChoice::Run);
}