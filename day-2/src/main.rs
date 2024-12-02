// level strictly increasing/decreasing
// two numbers differ by at least 1 and at most 3
use std::fs;

fn main() {
    // I know it exists so wont bother with the error handling
    let contents: String = fs::read_to_string("input.txt").unwrap();

    let mut safe_count: usize = 0;

    for line in contents.lines() {
        let mut error_count: usize = 0;

        let numbers: Vec<isize> = line
            .split_whitespace()
            .map(|n| n.parse::<isize>().unwrap())
            .collect();
        if validate_difference(&numbers, &mut error_count)
            && validate_order(&numbers, &mut error_count)
        {
            safe_count += 1;
        }
    }

    println!("There are {} safe reports", safe_count);
}

/// Takes in a vector of numbers and validates that they follow the difference rules.
fn validate_difference(level: &Vec<isize>, error_count: &mut usize) -> bool {
    for i in 1..level.len() {
        let diff: isize = level.get(i).unwrap() - level.get(i - 1).unwrap();
        if diff > 3 || diff < -3 || diff == 0 {
            *error_count += 1;
            if *error_count > 1 {
                return false;
            }
        }
    }

    return true;
}

fn validate_order(level: &Vec<isize>, error_count: &mut usize) -> bool {
    let asc: bool = if level[0] > level[1] { false } else { true };
    for i in 1..level.len() {
        match asc {
            true => {
                if level[i] < level[i - 1] {
                    *error_count += 1;
                    if *error_count > 1 {
                        return false;
                    }
                }
            }
            false => {
                if level[i] > level[i - 1] {
                    *error_count += 1;
                    if *error_count > 1 {
                        return false;
                    }
                }
            }
        }
    }
    return true;
}
