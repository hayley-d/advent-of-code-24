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
}
