use day_4::matrix::Matrix;
use std::fs;

fn main() {
    let patterns: Vec<&str> = vec!["XMAS", "SAMX"];

    let patterns2: Vec<&str> = vec!["XMAS"];

    let content: String = fs::read_to_string("input.txt").unwrap();

    let matrix: Matrix = Matrix::new(content);
    println!(
        "There are {} total overlapping matches",
        matrix.calculate_matches(patterns)
    );
}

fn max(num1: isize, num2: isize) -> isize {
    if num1 > num2 {
        return num1;
    }
    return num2;
}

fn min(num1: isize, num2: isize) -> isize {
    if num1 > num2 {
        return num2;
    }
    return num1;
}
