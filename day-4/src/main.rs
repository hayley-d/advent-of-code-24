use aho_corasick::{AhoCorasick, PatternID};
use day_4::matrix::Matrix;
use std::fs;

fn main() {
    let patterns: Vec<&str> = vec!["XMAS", "SAMX"];

    let content: String = fs::read_to_string("input.txt").unwrap();

    let matrix: Matrix = Matrix::new(content);
    let h_total = matrix.h_search(&patterns);
    println!("There are {} horazontil matches", h_total);

    let v_total = matrix.v_search(&patterns);
    println!("There are {} virticle matches", v_total);

    test_diagonal();
}

fn test_diagonal() {
    for k in 0..10 {
        for j in 0..=k {
            let i = k - j;
            print!("[{}][{}]", i, j);
        }
        println!("\n");
    }

    for k in 0..20 {
        for j in 0..=k {
            let i = k - j;
            if i < 10 && j < 10 {
                print!("[{}][{}]", i, j);
            }
        }
        println!("\n");
    }
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
