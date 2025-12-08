use std::fs;

enum Source {
    TEST,
    PUZZLE,
}

struct IDs {
    range_fresh: Vec<(usize, usize)>,
    ids_available: Vec<usize>,
}

fn read_input(source: &Source) -> IDs {
    let content = match source {
        Source::TEST => fs::read_to_string("data/test.txt").expect("should read TEST file"),
        Source::PUZZLE => fs::read_to_string("data/puzzle.txt").expect("should read PUZZLE file"),
    };

    let mut fresh: Vec<(usize, usize)> = Vec::new();
    let mut available: Vec<usize> = Vec::new();

    for line in content.lines() {
        if line.contains('-') {
            let (l, r) = line.split_once('-').unwrap();
            fresh.push((l.parse().unwrap(), r.parse().unwrap()));
        } else {
            let id = line.parse();
            match id {
                Ok(v) => available.push(v),
                Err(_e) => continue,
            }
        }
    }

    let result = IDs {
        range_fresh: fresh,
        ids_available: available,
    };

    result
}



fn helper_is_in_range(ranges: &Vec<(usize, usize)>, value: usize) -> usize {
    for r in ranges {
        if value >= r.0 && value <= r.1 {
            return 1;
        }
    }

    0
}

/* fn helper_crop_range(ranges: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut ranges_cropped = ranges.clone();
    let len = ranges_cropped.len();
    
    let mut mark_to_delete: Vec<usize> = Vec::new();

    for i in 0..len {

        for j in 0..len {
            if i == j {
                continue;
            }
            
            let rj = ranges_cropped[j];

            // check upper case
            if (rj.0..=rj.1).contains(&ranges_cropped[i].1) {

                // check also lower
                // is its both, mark to delete
                if (rj.0..=rj.1).contains(&ranges_cropped[i].0) {
                    println!("delete!!");
                    mark_to_delete.push(i);
                    continue;
                }

                let new = rj.0-1;
                
                let ri_ = ranges_cropped[i];
                ranges_cropped[i].1 = new;
                let ri = ranges_cropped[i];


                if ranges_cropped[i].0 > ranges_cropped[i].1 {

                    println!("\nCHANGE DETECTED HIGH\n");
                    println!("value to change: {:?}[1]", ri_);
                    println!("{} is in {:?}", ri_.1, rj);
                    println!("{} -> {}", ri_.1, new);
                    println!("new value: {:?}, diff: {}\n", ri, ri.1 as isize - ri.0 as isize);

                    println!("!!!!!!!!! {:?}\n", ranges_cropped[i]);
                }
                
            }

            // check lower case
            if (rj.0..=rj.1).contains(&ranges_cropped[i].0) {
                
                // check also higher
                // is its both, mark to delete
                if (rj.0..=rj.1).contains(&ranges_cropped[i].1) {
                    println!("delete!!");
                    mark_to_delete.push(i);
                    continue;
                }

                let new = rj.1+1;

                let ri_ = ranges_cropped[i];
                ranges_cropped[i].0 = new;
                let ri = ranges_cropped[i];

                if ranges_cropped[i].0 > ranges_cropped[i].1 {

                    println!("\nCHANGE DETECTED LOWER\n");
                    println!("value to change: {:?}[0]", ri_);
                    println!("{} is in {:?}", ri_.0, rj);
                    println!("{} -> {}", ri_.0, new);
                    println!("new value: {:?}, diff: {}\n", ri, ri.1 as isize - ri.0 as isize);

                    println!("!!!!!!!!! {:?}\n", ranges_cropped[i]);
                }

            }
            
            // check if makes sense
            // if ranges_cropped[i].0 > ranges_cropped[i].1 {
            //     println!("!!!!!!!!! {:?}", ranges_cropped[i]);
            // }
            
            
        }
    }

    println!("mark_to_delete: {:?}", mark_to_delete);

    for i in mark_to_delete.iter().rev() {
        ranges_cropped.remove(*i);
    }

    ranges_cropped
} */

fn part1(source: &Source) -> usize {
    let input = read_input(source);
    // println!("fresh    : {:?}", input.fresh);
    // println!("available: {:?}", input.available);

    let mut result = 0;
    for id in input.ids_available {
        result += helper_is_in_range(&input.range_fresh, id);
    }

    println!("part1 result: {}", result);

    result
}

// #[derive(Debug)]
// #[derive(PartialEq)]

// enum OverlapReason {
//     LowInside,
//     HighInside,
// }

/* fn _helper_check_overlap(ri: &(usize, usize), rj: &(usize, usize)) -> Vec<OverlapReason> {
    let mut result = Vec::new();

    if (rj.0..=rj.1).contains(&ri.0) {
        result.push(OverlapReason::LowInside);
    }

    if (rj.0..=rj.1).contains(&ri.1) {
        result.push(OverlapReason::HighInside);
    }

    // if result.len() > 0 {
    //     println!("there is overlap");
    //     println!("ri: {:?}", ri);
    //     println!("rj: {:?}", rj);
    //     println!("reason: {:?}\n", result);
    // }
    
    result
} */


fn part2(source: &Source) -> usize {
    
    let input = read_input(source);
    let mut ranges = input.range_fresh;
    ranges.sort();


    let mut total = 0;

    let mut current:isize = -1;
    for ri in ranges {
        let mut l = ri.0 as isize;
        let r = ri.1 as isize;
        
        // if current id is >= `l`, then move `l`
        if current >= l {
            l = current + 1;
        }
        if l <= r {
            total += r - l + 1;
        }
        current = std::cmp::max(current, r);
        
    }
    
    println!("part2 result: {}", total);
    
    total as usize
}

fn main() {
    let _r1_t = part1(&Source::TEST);
    let _r1_p = part1(&Source::PUZZLE);
    
    println!("\n---------------------------\n");

    // let r2_t = part2(&Source::TEST);
    let _r2_p = part2(&Source::PUZZLE);

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
        assert_eq!(part1(&Source::TEST), 3)
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(&Source::TEST), 43)
    }
}
