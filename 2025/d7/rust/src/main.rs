use std::{collections::HashSet, fs};

enum Source {
    TEST,
    PUZZLE,
}

#[derive(Debug)]
enum Operation {
    Mult(Vec<usize>),
    Sum(Vec<usize>),
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

fn helper_beams_increment(beams: &mut Vec<(usize, usize)>) {
    for i in 0..beams.len() {
        beams[i].0 += 1;
    }
}

fn helper_beams_propagate(data: &mut Vec<String>, beams: &mut Vec<(usize, usize)>, count: &mut usize) {
    for i in 0..beams.len() {
        let bi = beams[i];
        
        // get element
        let c = helper_get_element(&data, bi.0, bi.1);

        if c == '.' {
            helper_set_elemenet(data, bi.0, bi.1);
        } else if c == '^' {
            println!("hit ^: ({}, {})", bi.0, bi.1);
            beams[i].1 -= 1;
            
            let mut bnew = bi.clone();
            bnew.1 += 1;
            beams.push(bnew);
            
            *count += 1;
        }
    }
    
    // make beams unique
    let beams_unique: HashSet<(usize, usize)> = beams.iter().cloned().collect();
    let mut beams_ordered: Vec<(usize, usize)> = beams_unique.into_iter().collect(); 
    beams_ordered.sort();
    
    *beams = beams_ordered;
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
    let len_j = data[0].chars().count();
    
    
    // find position of S
    let beam_j = data[0].chars().position(|c| c == 'S').unwrap();
    let mut beams: Vec<(usize, usize)> = Vec::new();

    beams.push((0, beam_j));
    
    // propagate beam

    let mut count_splits = 0;
    for _ in 0..len_i {
        helper_beams_increment(&mut beams);
        helper_beams_propagate(&mut data, &mut beams, &mut count_splits);
            
        helper_data_print(&data);
    }
    
    println!("count: {}", count_splits);
    println!("beams({}): {:?}", beams.len(), beams);

    let beams_unique: HashSet<(usize, usize)> = beams.into_iter().collect();
    println!("beams_unique({}): {:?}", beams_unique.len(), beams_unique);


    100
}

fn part2(source: &Source) -> usize {
    println!("part2");

    200
}

fn main() {
    let _r1_t = part1(&Source::TEST);
    let _r1_p = part1(&Source::PUZZLE);

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
        assert_eq!(part2(&Source::TEST), 3263827)
    }
}
