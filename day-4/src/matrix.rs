use aho_corasick::AhoCorasick;

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

        if matrix.len() > matrix[0].len() {
            let diff: usize = matrix.len() - matrix[0].len();

            for row in &mut matrix {
                for _ in 0..diff {
                    row.push('\0');
                }
            }
        } else {
            let diff: usize = matrix[0].len() - matrix.len();

            for _ in 0..diff {
                let mut placeholder: Vec<char> = Vec::new();
                for _ in 0..matrix[0].len() {
                    placeholder.push('\0');
                }
                matrix.push(placeholder);
            }
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

        for col in 0..self.matrix.len() {
            let mut chars: Vec<char> = Vec::new();

            for row in 0..self.matrix.len() {
                chars.push(self.matrix[row][col]);
            }

            let line: String = chars.iter().collect();
            let ac = AhoCorasick::builder()
                //.ascii_case_insensitive(true)
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
        let dimensions: isize = self.matrix.len() as isize;

        for k in 0..(2 * dimensions) {
            let mut chars: Vec<char> = Vec::new();
            for j in 0..=k {
                let i = k - j;
                if i < dimensions && j < dimensions {
                    chars.push(self.matrix[i as usize][j as usize]);
                }
            }

            let line: String = chars.iter().collect();
            let ac = AhoCorasick::builder()
                //.ascii_case_insensitive(true)
                .build(patterns)
                .unwrap();

            let matches: usize = ac
                .find_overlapping_iter(&line)
                .map(|mat| mat.pattern())
                .count();

            total += matches;
        }

        for k in 0..2 * dimensions {
            let mut chars: Vec<char> = Vec::new();

            for j in 0..=k {
                let i: isize = (dimensions - 1) - (k - j);
                if i >= 0 && i < dimensions && j < dimensions {
                    chars.push(self.matrix[i as usize][j as usize]);
                }
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

    pub fn calculate_matches(&self, patterns: Vec<&str>) -> usize {
        let d: usize = self.diagonal_search(&patterns);
        println!("Diagonals: {}", d);
        return self.h_search(&patterns) + self.v_search(&patterns) + d;
    }

    pub fn xmas(&self) {
        let mut total: usize = 0;
        for row in 0..self.matrix.len() as isize {
            for col in 0..self.matrix.len() as isize {
                if self.matrix[row as usize][col as usize] == 'A' {
                    if row - 1 >= 0
                        && col - 1 >= 0
                        && (row + 1 as isize) < self.matrix.len() as isize
                        && col + 1 < self.matrix.len() as isize
                    {
                        let row = row as usize;
                        let col = col as usize;

                        if (self.matrix[row + 1][col - 1] == 'S'
                            && self.matrix[row - 1][col + 1] == 'M')
                            || (self.matrix[row + 1][col - 1] == 'M'
                                && self.matrix[row - 1][col + 1] == 'S')
                        {
                            if self.matrix[row - 1][col - 1] == 'M'
                                && self.matrix[row + 1][col + 1] == 'S'
                                || (self.matrix[row - 1][col - 1] == 'S'
                                    && self.matrix[row + 1][col + 1] == 'M')
                            {
                                total += 1;
                            }
                        }
                    }
                }
            }
        }
        println!("X-MAS: {}", total);
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
