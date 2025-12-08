use std::fs::{self, read};

enum Source {
    TEST,
    PUZZLE,
}

fn read_input(source: &Source) -> Vec<Vec<char>> {
    let content = match source {
        Source::TEST => fs::read_to_string("data/test.txt").expect("should read TEST file"),
        Source::PUZZLE => fs::read_to_string("data/puzzle.txt").expect("should read PUZZLE file"),
    };
    

    let mut result: Vec<Vec<char>> = Vec::new();
    
    for line in content.lines() {
        result.push(line.chars().collect());
    }

    result
}

fn helper_pick_elem(input: &Vec<Vec<char>>, i: isize, j: isize) -> char {
    const DEFAULT_VALUE: char = '.';

    if i < 0 || j < 0 {
        return DEFAULT_VALUE;
    }
    
    if i >= input.len() as isize {
        return DEFAULT_VALUE;
    }

    if j >= input[0].len() as isize {
        return DEFAULT_VALUE;
    }

    *input[i as usize].iter().nth(j as usize).unwrap_or(&DEFAULT_VALUE)
}

fn helper_pick_elem_num(input: &Vec<Vec<char>>, i: isize, j: isize) -> u64 {

    let c = helper_pick_elem(input, i, j);
    
    if c == '@' {
        return 1;
    } else {
        return 0;
    }
}


fn helper_check_adjacent(input: &Vec<Vec<char>>, i: isize, j: isize) -> bool {
    
    if helper_pick_elem(input, i, j) == '.' {
        return false;
    }

    let mut count:u64 = 0;
    
    count += helper_pick_elem_num(input, i-1, j-1);
    count += helper_pick_elem_num(input, i-1, j+1);
    count += helper_pick_elem_num(input, i+1, j-1);
    count += helper_pick_elem_num(input, i+1, j+1);
    count += helper_pick_elem_num(input, i, j-1);
    count += helper_pick_elem_num(input, i, j+1);
    count += helper_pick_elem_num(input, i-1, j);
    count += helper_pick_elem_num(input, i+1, j);
    
    let result = count < 4;
    // println!("count ({}, {}): {} | {}", i, j, count, result);
    
    result
}

fn part1(source: &Source) -> u64 {
    let input = read_input(source);
    
    let mut count = 0;
    for (i, vi) in input.iter().enumerate() {
        for (j, _) in vi.iter().enumerate() {
            if helper_check_adjacent(&input, i as isize, j as isize) {
                count += 1;
            }
        }
    }
    
    count
}

fn helper_modify_elem(input: &mut Vec<Vec<char>>, i: usize, j: usize, new_char: char) {
    let inner_vec: &mut Vec<char> = &mut input[i];
    inner_vec[j] = new_char;
}

fn helper_part2_find_rolls(input: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut access_ok = Vec::new();

    for (i, vi) in input.iter().enumerate() {
        for (j, _) in vi.iter().enumerate() {
            if helper_check_adjacent(&input, i as isize, j as isize) {
                access_ok.push((i, j));
            }
        }
    }
    
    access_ok
}

fn helper_part2_remove_rools(input: &mut Vec<Vec<char>>, access_ok: &Vec<(usize, usize)>) {
    for (i, j) in access_ok {
        helper_modify_elem(input, *i, *j, '.');
    }
}


fn helper_part2_print_rolls(input: &Vec<Vec<char>>) {
    for ii in input {
        let line: String = ii.iter().collect();
        println!("{}", line);
    }
}

fn part2(source: &Source) -> u64 {
    let mut input = read_input(source);
    
    let mut total = 0;

    loop {
        let access_ok = helper_part2_find_rolls(&input);

        if access_ok.len() == 0 {
            break;
        }

        helper_part2_remove_rools(&mut input, &access_ok);
        helper_part2_print_rolls(&input);

        total += access_ok.len() as u64;
    }
    
    total
}

fn main() {
    let r1_t = part1(&Source::TEST);
    let r2_t = part2(&Source::TEST);

    let r1_p = part1(&Source::PUZZLE);
    let r2_p = part2(&Source::PUZZLE);

    println!("\nTEST");
    println!("result part1: {}", r1_t);
    println!("result part2: {}", r2_t);

    println!("\nPUZZLE");
    println!("result part1: {}", r1_p);
    println!("result part2: {}", r2_p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1(&Source::TEST), 13)
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(&Source::TEST), 43)
    }
}

// line 4: 976546353526
