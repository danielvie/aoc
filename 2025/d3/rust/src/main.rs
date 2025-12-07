use std::{fs, vec};
use itertools::Itertools;

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

    for (i, line) in input.lines().enumerate() {
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

fn pick_increment(v: &mut Vec<u64>, lim: u64) -> bool {
    let n = v.len();
    
    v[n-1] += 1;
    
    // adjust overflow
    for i in 0..n {
        let ii = n - i - 1;
        
        if v[ii] > lim {
            v[ii] = 0;
            v[ii - 1] += 1;
        }
    }
    
    let mut refe = 0;
    for i in 0..v.len() {
        let ii = n - i - 1;
    }

    let total: u64 = v.iter().sum();

    false
}

fn pick_n(line: &str, n: u64) -> Vec<Vec<u64>> {
    
    let mut pool = Vec::new();
    
    for i in 0..line.len() {
        pool.push(i);
    }

    let n = line.len() as u64;
    println!("line.len(): {}", n);

    let mut pointers = Vec::new();
    for i in 0..2 {
        pointers.push(i);
    }
    
    let mut result = Vec::new();
    for _ in 0..5*n {
        result.push(pointers.clone());
        pick_increment(&mut pointers, n);
    }
    
    println!("result: {:?}", result);

    result
}



fn increment_helper(pointers: &mut Vec<u64>, max: usize, pos: usize) -> bool {

    if pos == 0 && (pointers[pos] > max as u64) {
        return false;
    }

    pointers[pos] += 1;
    
    if pointers[pos] > max as u64 {
        if pos == 0 {
            return false;
        }
        let success = increment_helper(pointers, max-1, pos-1);
        if !success {
            return false;
        }

        pointers[pos] = pointers[pos-1]+1;
    }
    
    true
}

fn increment(pt: &mut Vec<u64>, max: usize) -> bool {
    let pos = pt.len()-1;

    increment_helper(pt, max, pos)
}

fn part2(source: &Source) -> u64 {
    
    // initialize pointers
    let mut pointers: Vec<u64> = Vec::new();
    let n = 2;
    for i in 0..n {
        pointers.push(i);
    }
    
    // create pool
    let pool = vec![0,1,2,3,4,5];

    // picker
    let picker = |pool_: &Vec<u64>, pts: &Vec<u64>| -> Vec<u64> {
        let mut pick = Vec::new();
        for p in pts {
            pick.push(pool_[*p as usize]);
        }
        
        pick
    };
    

    let mut pick = picker(&pool, &pointers);

    // increment pointers

    println!("pointers: {:?}", pointers);
    for _ in 0..20 {
        let res = increment(&mut pointers, 5);
        if res == false {
            break;
        }
        println!("pointers: {:?}", pointers);
    }

    // pick = picker(&pool, &pointers);

    // println!("pointers: {:?}", pointers);
    // println!("pool    : {:?}", pool);
    // println!("pick    : {:?}", pick);


    100
}

fn main() {
    // let r1_t = part1(&Source::TEST);
    let r2_t = part2(&Source::TEST);

    // let r1_p = part1(&Source::PUZZLE);
    // let r2_p = part2(&Source::PUZZLE);

    // println!("TEST");
    // println!("result part1: {}", r1_t);
    println!("result part2: {}", r2_t);

    // println!("PUZZLE");
    // println!("result part1: {}", r1_p);
    // println!("result part2: {}", r2_p);
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
        assert_eq!(part2(&Source::TEST), 3121910778619)
    }
}
