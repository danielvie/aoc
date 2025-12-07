use itertools::Itertools;
use std::fs;

enum Source {
    TEST,
    PUZZLE,
}

fn read_file(source: &Source) -> String {
    match source {
        Source::TEST => fs::read_to_string("data/test.txt").expect("should read TEST file"),
        Source::PUZZLE => fs::read_to_string("data/puzzle.txt").expect("should read PUZZLE file"),
    }
}

fn part1(source: &Source) -> u64 {
    let input = read_file(source);

    let mut bl;
    let mut br;

    let mut results: Vec<u64> = Vec::new();

    for line in input.lines() {
        let mut value = 0;
        for i in 0..line.len() {
            bl = &line[i..i + 1];

            for j in (i + 1)..line.len() {
                br = &line[j..j + 1];

                let s = bl.to_string() + br;
                let v = s.parse().unwrap();

                if value < v {
                    value = v;
                }
            }
        }

        results.push(value);
    }

    results.iter().sum()
}

fn max_finder(line: &str, pointers: &mut Vec<usize>, pos: usize) {
    let end = pointers[pos];

    // computing ini
    let mut ini = 0;
    if pos > 0 {
        ini = pointers[pos - 1] + 1;
    }

    // moving to left
    let mut maxi = end;
    let mut max = line.chars().nth(maxi).unwrap().to_digit(10).unwrap() as usize;
    let mut p = maxi;

    for _ in ini..end {
        p -= 1;
        let n: usize = line.chars().nth(p).unwrap().to_digit(10).unwrap() as usize;

        if max <= n {
            max = n;
            maxi = p;
        }
    }

    pointers[pos] = maxi;
}

// picker
fn picker(line: &str, pts: &Vec<usize>) -> u64 {
    let mut pick = Vec::new();
    for p in pts {
        let char = line.trim().chars().nth(*p as usize);
        pick.push(char.unwrap());
    }

    let result = pick.iter().join("").parse().unwrap();
    result
}

fn part2(source: &Source, number_of_elements: usize) -> u64 {
    let input = read_file(source);
    let n = number_of_elements;

    // start pointers
    let mut pointers: Vec<usize> = Vec::new();

    for i in 0..n {
        pointers.push(i);
    }

    let mut result: Vec<u64> = Vec::new();
    for line in input.lines() {
        // move pointers to end of line
        let line_len = line.len();

        for i in 0..n {
            pointers[i] = i;
        }

        let dif = line_len - pointers.len();
        for i in 0..pointers.len() {
            pointers[i] += dif;
        }

        for i in 0..n {
            max_finder(&line, &mut pointers, i);
        }

        let value = picker(&line, &pointers);

        result.push(value);
    }

    let soma = result.iter().sum();

    soma
}

fn main() {
    let r1_t = part1(&Source::TEST);
    let r2_t = part2(&Source::TEST, 12);

    let r1_p = part1(&Source::PUZZLE);
    let r2_p = part2(&Source::PUZZLE, 12);

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
        assert_eq!(part1(&Source::TEST), 357)
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(&Source::TEST, 12), 3121910778619)
    }

    #[test]
    fn part2_test_with_2() {
        assert_eq!(part2(&Source::TEST, 2), 357)
    }
}

// line 4: 976546353526
