use std::{fs};

enum Source {
    TEST,
    PUZZLE
}

struct Range {
    ini: u32,
    end: u32,
}

fn read_file(source: &Source) -> String {
    let content = match source {
        Source::TEST => fs::read_to_string("data/test.txt"),
        Source::PUZZLE => fs::read_to_string("data/puzzle.txt"),
    };
    
    match content {
        Ok(s) => s,
        Err(e) => panic!("problem here: {}", e)
    }
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
    
    print!("- {}-{}", item.ini, item.end);
    if invalid.len() == 0 {
        println!(" contains no invalid IDs");
    } else {
        print!(" contains {} invalid: ", invalid.len());
        for ii in &invalid {
            print!("{}, ", ii);
        }
        print!("\n");
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
    
    print!("- {}-{}", item.ini, item.end);
    if invalid.len() == 0 {
        println!(" contains no invalid IDs");
    } else {
        print!(" contains {} invalid: ", invalid.len());
        for ii in &invalid {
            print!("{}, ", ii);
        }
        print!("\n");
    }
    
    invalid.iter().sum()
}

fn part1(source: &Source) -> u64 {
    let content = read_file(&source);
    
    let mut ranges: Vec<Range> = Vec::new();

    let lines = content.lines();
    for li in lines {
        let rr = li.split(",");
        
        for ri in rr {
            if ri == "" {
                continue;
            }

            let mut parts = ri.split('-');
            
            // get parts
            let ini_s = parts.next().unwrap_or_default();
            let end_s = parts.next().unwrap_or_default();
            
            let ini = ini_s.parse::<u32>().unwrap();
            let end = end_s.parse::<u32>().unwrap();

            ranges.push(Range {ini, end});
        }
    }
    
    let mut result:u64 = 0;
    for ri in ranges {
        // println!("{} .. {}", ri.ini, ri.end);
        result += check_range(&ri) as u64;
    }
    
    println!("\nresult: {}", result);
    
    result
}

fn part2(source: &Source) -> u64 {
    let content = read_file(&source);
    
    let mut ranges: Vec<Range> = Vec::new();

    let lines = content.lines();
    for li in lines {
        let rr = li.split(",");
        
        for ri in rr {
            if ri == "" {
                continue;
            }

            let mut parts = ri.split('-');
            
            // get parts
            let ini_s = parts.next().unwrap_or_default();
            let end_s = parts.next().unwrap_or_default();
            
            let ini = ini_s.parse::<u32>().unwrap();
            let end = end_s.parse::<u32>().unwrap();

            ranges.push(Range {ini, end});
        }
    }
    
    let mut result:u64 = 0;
    for ri in ranges {
        result += check_range_part2(&ri) as u64;
    }
    
    println!("\nresult: {}", result);
    
    result
}

fn main() {
    part1(&Source::TEST);
    part2(&Source::PUZZLE);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part1_test() {
        assert_eq!(part1(&Source::TEST), 1227775554)
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(&Source::TEST), 4174379265)
    }

}