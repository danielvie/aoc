use std::fs;

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

fn part1(source: &Source) -> usize {
    let input = read_input(source);

    let mut collection = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        collection.push(parts);
    }

    let n = collection[0].len();

    let mut values = Vec::new();

    for i in 0..n {
        let mut temp = Vec::new();
        for el in &collection {
            temp.push(el[i]);
        }
        values.push(temp);
    }

    let mut numbers: Vec<usize> = Vec::new();
    let mut input_operations = Vec::new();

    for oper in &values {
        numbers.clear();
        for elem in oper {
            match elem.parse() {
                Ok(v) => numbers.push(v),
                Err(_e) => {
                    if *elem == "*" {
                        input_operations.push(Operation::Mult(numbers.clone()));
                    } else if *elem == "+" {
                        input_operations.push(Operation::Sum(numbers.clone()));
                    }
                }
            }
        }
    }

    println!("operations: {:?}", input_operations);

    // println!("values: {:?}", values);

    let mut total = 0;
    for op in input_operations {
        match &op {
            Operation::Sum(v) => {
                let value = v.iter().sum::<usize>();
                println!("{:?}: {}", op, value);
                total += value;
            }
            Operation::Mult(v) => {
                let value = v.iter().product::<usize>();
                println!("{:?}: {}", op, value);
                total += value;
            }
        }
    }

    let result = total;

    println!("result: {}", total);

    result
}

fn helper_picker(matrix: &Vec<String>, i: usize, j: usize) -> char {
    if i > matrix.len() {
        return ' ';
    }

    match matrix[i].chars().nth(j) {
        Some(v) => v,
        None => ' ',
    }
}

fn part2(source: &Source) -> usize {
    let content = read_input(source);

    println!("content: \n{}", content);

    // lets break all in chars()
    let mut matrix_char = Vec::new();
    for line in content.lines() {
        matrix_char.push(line.to_string());
    }

    // lets make all lines with the same len()
    let len_i = matrix_char.len();
    let len_j = matrix_char.iter().map(|s| s.len()).max().unwrap_or(0);

    
    let mut current = len_j;
    let mut numbers = Vec::new();
    let mut operations = Vec::new();

    loop {
        if current == 0 {
            break;
        }
        current -= 1;

        let j = current;
        let mut number_chars = Vec::new();
        for i in 0..len_i {
            let c = helper_picker(&matrix_char, i, j);
            number_chars.push(c);
        }

        let number;
        let last_char = number_chars[len_i-1];

        if last_char == '*' {
            number = number_chars.iter().take(len_i-1).collect::<String>().trim().parse::<usize>().unwrap();
            numbers.push(number);
            operations.push(Operation::Mult(numbers.clone()));
            numbers.clear();
        } else if last_char == '+' {
            number = number_chars.iter().take(len_i-1).collect::<String>().trim().parse::<usize>().unwrap();
            numbers.push(number);
            operations.push(Operation::Sum(numbers.clone()));
            numbers.clear();
        } else {
            if number_chars.iter().all(|c| *c == ' ') {
                continue;
            }
            number = number_chars.iter().collect::<String>().trim().parse::<usize>().unwrap();
            numbers.push(number);
        }
    }
    println!("operations: {:?}", operations);

    let mut result = 0;
    for op in operations {
        let value = match op {
            Operation::Mult(v) => v.iter().product::<usize>(),
            Operation::Sum(v) => v.iter().sum::<usize>(),
        };
        
        result += value;
    }
    
    println!("result part2: {}", result);

    result
}

fn main() {
    // let _r1_t = part1(&Source::TEST);
    // let _r1_p = part1(&Source::PUZZLE);

    let _r2_t = part2(&Source::TEST);
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
        assert_eq!(part1(&Source::TEST), 4277556)
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(&Source::TEST), 3263827)
    }
}
