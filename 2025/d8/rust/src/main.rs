use std::{fs, mem::discriminant};

enum Source {
    TEST,
    PUZZLE,
}

#[derive(Debug, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
    z: usize,
}

impl Pos {
    fn dist(&self, p: &Pos) -> f64 {
        let temp = self.x.abs_diff(p.x).pow(2) + self.y.abs_diff(p.y).pow(2) + self.z.abs_diff(p.z).pow(2);
        (temp as f64).sqrt()
    }
}

fn input_read(source: &Source) -> String {
    let content = match source {
        Source::TEST => fs::read_to_string("data/test.txt").expect("should read TEST file"),
        Source::PUZZLE => fs::read_to_string("data/puzzle.txt").expect("should read PUZZLE file"),
    };

    content
}

fn input_parse(input: &String) -> Vec<Pos> {
    
    let result = input.lines().map(|line| {
        let mut parts = line.split(",");
        
        let x = parts.next().unwrap().parse::<usize>().unwrap();
        let y = parts.next().unwrap().parse::<usize>().unwrap(); 
        let z = parts.next().unwrap().parse::<usize>().unwrap();
        
        Pos {x, y, z}
    }).collect();

    result
}

fn part1(source: &Source) -> usize {
    let input = input_read(&Source::TEST);
    let positions = input_parse(&input);
    
    let mut distances = Vec::new();
    

    for (i, p1) in positions.iter().enumerate() {
        for (j, p2) in positions.iter().enumerate().skip(i+1) {
            
            let d = p1.dist(p2);
            
            distances.push((d, (i, j)));
        }
    }
    
    distances.sort_by(|a, b| {
        a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Greater)
    });
    
    let distances_elem1: Vec<_> = distances.iter().filter(|&(_, (i, _))| *i == 0).collect();

    print!("\n");

    for (d, (i, j)) in &distances {
        println!("d: {:6.1}: {:3?} -> {:3?}", d, positions[*i], positions[*j]);
    }

    print!("\n");

    for (d, (i, j)) in distances_elem1 {
        println!("d: {:6.1}: {:3?} -> {:3?}", d, positions[*i], positions[*j]);
    }


    
    100
}

fn part2(source: &Source) -> usize {
    println!("part 2 not implemented");
    
    200
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
