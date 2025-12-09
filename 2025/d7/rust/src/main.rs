use std::{collections::HashSet, fs};

enum Source {
    TEST,
    PUZZLE,
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Beam {
    i: usize,
    j: usize,
    n: usize,
}

fn read_input(source: &Source) -> String {
    let content = match source {
        Source::TEST => fs::read_to_string("data/test.txt").expect("should read TEST file"),
        Source::PUZZLE => fs::read_to_string("data/puzzle.txt").expect("should read PUZZLE file"),
    };

    content
}

fn helper_data_print(data: &Vec<String>) {
    print!("\n");
    for d in data {
        println!("{}", d);
    }
    print!("\n");
}

fn helper_beams_increment(beams: &mut Vec<Beam>) {
    for i in 0..beams.len() {
        beams[i].i += 1;
    }
}

fn helper_beams_combine(beams: &mut Vec<Beam>) {
    let mut remove_items = Vec::new();
    let len = beams.len();
    for i in 0..len {
        let bi = beams[i];
        for j in 0..len {
            if i == j {
                continue;
            }

            let bj = beams[j];
            if bi.i == bj.i && bi.j == bj.j {

                beams[i].n = beams[i].n + bj.n;
                beams[j].n = 0;
                
                remove_items.push(j);
            }
        }
    }
    
    beams.retain(|b| b.n != 0);
}

fn helper_beams_propagate(data: &mut Vec<String>, beams: &mut Vec<Beam>, count_splits: &mut usize) {

    for i in 0..beams.len() {
        let bi = beams[i];
        
        let c = helper_get_element(&data, bi.i, bi.j);

        if c == '.' {
            helper_set_elemenet(data, bi.i, bi.j);
        } else if c == '^' {
            beams[i].j -= 1;
            
            let mut bnew = bi.clone();
            bnew.j += 1;
            beams.push(bnew);
            
            *count_splits += 1;
        }
    }
    
    helper_beams_combine(beams);
}

fn helper_get_element(data: &Vec<String>, i: usize, j: usize) -> char {
    if i >= data.len() {
        return '.';
    }

    data[i].chars().nth(j).unwrap_or('.')
}

fn helper_set_elemenet(data: &mut Vec<String>, i: usize, j: usize) {
    if i < data.len() {
        let new_char = '|';
        data[i].replace_range(j..j+1, &new_char.to_string());
    }
}

fn part1(source: &Source) -> usize {
    let content = read_input(source);
    

    // find sizes
    let mut data = Vec::new();
    
    for line in content.lines() {
        data.push(line.to_string());
    }
    
    let len_i = data.len();
    // let len_j = data[0].chars().count();
    
    
    // find position of S
    let beam_j = data[0].chars().position(|c| c == 'S').unwrap();
    let mut beams: Vec<Beam> = Vec::new();

    beams.push(Beam {i: 0, j: beam_j, n: 1});
    
    // propagate beam
    let mut count_splits = 0;
    for _ in 0..len_i {
        helper_beams_increment(&mut beams);
        helper_beams_propagate(&mut data, &mut beams, &mut count_splits);
            
        helper_data_print(&data);
    }
    
    println!("count: {}", count_splits);
    println!("beams.len(): {}", beams.len());

    let beams_unique: HashSet<Beam> = beams.into_iter().collect();
    println!("beams_unique({}): {:?}", beams_unique.len(), beams_unique);

    let result = count_splits;

    result
}

fn part2(source: &Source) -> usize {
    let content = read_input(source);
    

    // find sizes
    let mut data = Vec::new();
    
    for line in content.lines() {
        data.push(line.to_string());
    }
    
    let len_i = data.len();
    
    
    // find position of S
    let beam_j = data[0].chars().position(|c| c == 'S').unwrap();
    let mut beams: Vec<Beam> = Vec::new();

    beams.push(Beam {i: 0, j: beam_j, n: 1});
    
    // propagate beam

    let mut count_splits = 0;
    for _ in 0..len_i {
        helper_beams_increment(&mut beams);
        helper_beams_propagate(&mut data, &mut beams, &mut count_splits);
            
        // helper_data_print(&data);
    }
    
    let beams_n_sum: usize = beams.iter().map(|b| b.n).sum();

    println!("count: {}", count_splits);
    println!("beams.len(): {}", beams.len());
    println!("beams.n.sum(): {}", beams_n_sum);

    let result = beams_n_sum;

    result
}

fn main() {
    let _r1_t = part1(&Source::TEST);
    // let _r1_p = part1(&Source::PUZZLE);

    // let _r2_t = part2(&Source::TEST);
    // let _r2_p = part2(&Source::PUZZLE);

    // println!("\nTEST");
    // println!("result part1: {}", r1_t);
    // println!("result part2: {}", r2_t);

    // println!("\nPUZZLE");
    // println!("result part1: {}", r1_p);
    // println!("result part2: {}", r2_p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1(&Source::TEST), 21)
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(&Source::TEST), 40)
    }
}
