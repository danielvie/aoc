
#[derive(Debug, Clone, Copy)]
enum FileChoice { Test, Test2, Puzzle, }

#[derive(Debug, PartialEq)]
enum State { DO, DONT, }


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

fn process_conditional(s: &str) -> usize {
    let mut pos = 0;
    let mut total = 0;
    let mut state = State::DO;
    while let Some(loc) = get_next(&s[pos..]) {
        pos += loc;
        match &s[pos..pos+3] {
            "mul" => {
                if state == State::DO {
                    if let Some(product) = multiply(&s[pos + 4..]) {
                        total += product;
                    }
                }
            }
            "do(" => state = State::DO,
            "don" => state = State::DONT,
            _ => panic!("bad pattern: {}", &s[pos..pos + 3]),
        }
        pos += 4;
    }
    
    total
}

fn multiply(s: &str) -> Option<usize> {
    // compute left
    let (left, rest) = s.split_once(',')?;
    let left = left.parse::<usize>().ok()?;
    
    // compute right
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

fn get_puzzle(choice: FileChoice) -> &'static str {
    match choice {
        FileChoice::Test => include_str!("../../data/test.txt"),
        FileChoice::Test2 => include_str!("../../data/test2.txt"),
        FileChoice::Puzzle => include_str!("../../data/puzzle.txt"),
    }
}

fn run(choice: FileChoice) {
    let puzzle = get_puzzle(choice);

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
    run(FileChoice::Puzzle);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_multiply() {
        assert_eq!(Some(1), multiply("1,1)"));
        assert_eq!(Some(115), multiply("23,5)"));
        assert_eq!(None, multiply("23,5]"));
        assert_eq!(Some(15), multiply("3,5)jkdfsdsee**"));
    }

    #[test]
    fn test_part1() {
        
        let puzzle_test = get_puzzle(FileChoice::Test2);

        assert_eq!(161, process(&puzzle_test));
        assert_eq!(48, process_conditional(&puzzle_test));
    }
}