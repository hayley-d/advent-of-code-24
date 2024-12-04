use aho_corasick::{AhoCorasick, MatchKind, PatternID};

pub struct Matrix {
    matrix: Vec<Vec<char>>,
}

impl Matrix {
    pub fn new(contents: String) -> Self {
        let mut matrix: Vec<Vec<char>> = Vec::new();
        for line in contents.lines() {
            let mut row: Vec<char> = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            matrix.push(row);
        }

        return Matrix { matrix };
    }

    pub fn get(&self, row: usize, col: usize) -> Option<char> {
        if row > self.matrix.len() || col > self.matrix[0].len() {
            return None;
        }

        return Some(self.matrix[row][col]);
    }

    pub fn h_search(&self, patterns: &Vec<&str>) -> usize {
        let mut total: usize = 0;
        for row in &self.matrix {
            let line: String = row.iter().collect();

            let ac = AhoCorasick::builder()
                .ascii_case_insensitive(true)
                .build(patterns)
                .unwrap();

            let matches: usize = ac
                .find_overlapping_iter(&line)
                .map(|mat| mat.pattern())
                .count();

            total += matches;
        }

        return total;
    }

    pub fn get_rows(&self) -> usize {
        return self.matrix.len();
    }

    pub fn get_cols(&self) -> usize {
        return self.matrix[0].len();
    }

    pub fn v_search(&self, patterns: &Vec<&str>) -> usize {
        let mut total: usize = 0;

        for col in 0..self.get_cols() {
            let mut chars: Vec<char> = Vec::new();

            for row in 0..self.get_rows() {
                chars.push(self.matrix[row][col]);
            }

            let line: String = chars.iter().collect();
            let ac = AhoCorasick::builder()
                .ascii_case_insensitive(true)
                .build(patterns)
                .unwrap();

            let matches: usize = ac
                .find_overlapping_iter(&line)
                .map(|mat| mat.pattern())
                .count();

            total += matches;
        }

        return total;
    }

    pub fn diagonal_search(&self, patterns: &Vec<&str>) -> usize {
        let mut total: usize = 0;

        let max_rows: usize = self.get_rows() - 1;
        let cols: usize = self.get_cols() - 1;

        let mut chars: Vec<char> = Vec::new();

        for max in 0..cols {
            let mut i = 0;
            let mut j = max;
            let mut offset = 0;

            if max > max_rows {
                offset = max - max_rows;
                j = max_rows;
            }

            while i <= max && j >= 0 {
                if i + offset < cols && j < max_rows {
                    chars.push(self.matrix[j][i + offset]);
                    println!("[{}][{}]", j, i + offset);
                } else {
                    break;
                }
                i += 1;
                j -= 1;
            }
        }

        return total;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_diagonal() {
        let max_rows: isize = 3;
        let cols: isize = 5;

        for max in 0..cols {
            let mut i: isize = 0;
            let mut j: isize = max;
            let mut offset: isize = 0;

            if max > max_rows {
                offset = max - max_rows;
                j = max_rows as isize;
            }

            while i <= max && j >= 0 {
                if i + offset < cols && j < max_rows {
                    println!("[{}][{}]", j, i + offset);
                } else {
                    break;
                }
                i += 1;
                j -= 1;
            }
        }
    }
}
