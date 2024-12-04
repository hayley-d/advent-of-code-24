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
}
