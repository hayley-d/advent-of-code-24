use regex::Regex;
use std::fs;

fn main() {
    let contents: String = fs::read_to_string("input.txt").unwrap();
    let reg = Regex::new(r"mul\((\d{1,4}),(\d{1,4})\)").unwrap();
    let matches: Vec<_> = reg.find_iter(&contents).map(|m| m.as_str()).collect();
    let mut total: usize = 0;
    for m in matches {
        total += get_mul(m);
    }

    println!("The total mul is {}", total);
}

fn get_mul(match_string: &str) -> usize {
    let reg = Regex::new(r"\d{1,4}").unwrap();
    let numbers: Vec<usize> = reg
        .find_iter(&match_string)
        .map(|m| m.as_str().parse::<usize>().unwrap())
        .collect();
    return numbers.iter().fold(1, |acc, n| acc * n);
}
