
#[derive(Debug, Clone, Copy)]
enum FileChoice { Test, Puzzle, }

fn get_puzzle(choice: FileChoice) -> &'static str {
    match choice {
        FileChoice::Test => include_str!("../../data/test.txt"),
        FileChoice::Puzzle => include_str!("../../data/puzzle.txt"),
    }
}

#[derive(Debug)]
struct Pair {
    x: i32,
    y: i32,
}

fn part1(s: &str) -> usize {
    18
}

fn part2(s: &str) -> usize {
    9
}

fn create_dir() -> Vec<Pair> {
    // get all directions
    let mut directions: Vec<Pair> = Vec::new();
    for i in -1..2 {
        for j in -1..2 {
            directions.push(Pair{ x:i, y:j });
        }
    }
    
    directions
}

fn run(choice: FileChoice) {
    let puzzle = get_puzzle(choice);

    // create directions
    let directions = create_dir();
    print!("{:?}\n", directions);

    // compute
    println!("{:?}", part1(&puzzle));
}

fn main() {
    println!("Test:");
    run(FileChoice::Test);

    // println!("\nPuzzle:");
    // run(FileChoice::Puzzle);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let puzzle_test = get_puzzle(FileChoice::Test);
        assert_eq!(18, part1(&puzzle_test));
    }

    #[test]
    fn test_part2() {
        let puzzle_test = get_puzzle(FileChoice::Test);
        assert_eq!(9, part2(&puzzle_test));
    }
}