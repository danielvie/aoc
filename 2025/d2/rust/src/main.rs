use std::fs;

enum Source {
    TEST,
    PUZZLE
}

struct Range {
    ini: u32,
    end: u32,
}

fn read_file(source: &Source) -> String {
    match source {
        Source::TEST => fs::read_to_string("data/test.txt").expect("should read TEST file"),
        Source::PUZZLE => fs::read_to_string("data/puzzle.txt").expect("should read PUZZLE file"),
    }.trim().replace("\r\n", "")
}

fn check_isvalid(n: u32) -> bool {
    let s = n.to_string();
    let len = s.len();
    
    if len % 2 == 1 {
        return false;
    }
    
    let (l, r) = s.split_at(len/2);
    if l == r {
        return true
    }

    false
}

fn check_range(item: &Range) -> u32 {
    let mut invalid: Vec<u32> = Vec::new();

    for n in item.ini ..= item.end {
        if check_isvalid(n) {
            invalid.push(n);
        }
    }
    
    invalid.iter().sum()
}

fn check_isvalid_part2(n: u32) -> bool {
    let s = n.to_string();
    let len = s.len();
    
    for n in 1..len {
        let mask = s.split_at(n).0;
        
        let mut rr = s.split(mask);
        
        if rr.all(|ri| ri == "") {
            return true
        }
    }
    
    false
}

fn check_range_part2(item: &Range) -> u32 {
    let mut invalid: Vec<u32> = Vec::new();

    for n in item.ini ..= item.end {
        if check_isvalid_part2(n) {
            invalid.push(n);
        }
    }
    
    invalid.iter().sum()
}

fn part1(ranges: &Vec<Range>) -> u64 {
    let mut result:u64 = 0;
    for ri in ranges {
        result += check_range(&ri) as u64;
    }
    
    result
}

fn part2(ranges: &Vec<Range>) -> u64 {
    let mut result:u64 = 0;
    for ri in ranges {
        result += check_range_part2(&ri) as u64;
    }
    
    result
}

fn parse_input(input: &str) -> Vec<Range> {
    input
        .trim()
        .split(',')
        .map(|segment| {
            let mut parts = segment.split('-');
            let ini = parts.next().unwrap().parse().unwrap();
            let end = parts.next().unwrap().parse().unwrap();
            Range {ini, end}
        })
        .collect()
}

fn main() {
    let input_test = read_file(&Source::TEST);
    let input_puzzle = read_file(&Source::PUZZLE);

    let ranges_t = parse_input(&input_test);
    let ranges_p = parse_input(&input_puzzle);
    
    let r1_t = part1(&ranges_t);
    let r2_t = part2(&ranges_t);

    let r1_p = part1(&ranges_p);
    let r2_p = part2(&ranges_p);
    
    println!("TEST");
    println!("result part1: {}", r1_t);
    println!("result part2: {}", r2_t);

    println!("PUZZLE");
    println!("result part1: {}", r1_p);
    println!("result part2: {}", r2_p);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part1_test() {
        let input = read_file(&Source::TEST);
        let ranges = parse_input(&input);
        assert_eq!(part1(&ranges), 1227775554)
    }

    #[test]
    fn part2_test() {
        let input = read_file(&Source::TEST);
        let ranges = parse_input(&input);
        assert_eq!(part2(&ranges), 4174379265)
    }

}