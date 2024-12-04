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
}
