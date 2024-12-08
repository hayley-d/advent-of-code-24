use day_6::Matrix;
use std::fs;

fn main() {
    let contents: String = fs::read_to_string("input.txt").unwrap();
    let mut matrix: Matrix = Matrix::create(contents);

    println!("{}", matrix);
    let (row, col) = matrix.get_gaurd();
    matrix.move_up(row, col);
    println!("{}", matrix);

    println!("The gaurd went to {} locations", matrix.count_x());
}
