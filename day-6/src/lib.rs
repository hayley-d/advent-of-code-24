use std::fmt::Display;

pub struct Matrix {
    matrix: Vec<Vec<char>>,
}

impl Matrix {
    pub fn new(rows: usize) -> Self {
        return Matrix {
            matrix: Vec::with_capacity(rows),
        };
    }

    pub fn get(&self, row: usize, col: usize) -> Option<char> {
        if row < self.matrix.len() && col < self.matrix[0].len() {
            return Some(self.matrix[row][col]);
        }

        return None;
    }

    pub fn create(content: String) -> Self {
        let mut rows: Vec<Vec<char>> = Vec::new();
        for line in content.lines() {
            let line: Vec<char> = line.chars().collect();
            rows.push(line);
        }

        return Matrix { matrix: rows };
    }

    pub fn can_move(&self, row: usize, col: usize) -> bool {
        match self.get(row, col) {
            Some(p) if p == '#' => false,
            Some(_) => true,
            None => false,
        }
    }

    pub fn get_gaurd(&self) -> (usize, usize) {
        for (i, row) in self.matrix.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if *c == '<' || *c == '>' || *c == '^' || *c == 'v' {
                    return (i, j);
                }
            }
        }
        return (0, 0);
    }

    fn rotate_gaurd(&mut self, row: usize, col: usize) {
        match self.matrix[row][col] {
            '^' => self.matrix[row][col] = '>',
            '>' => self.matrix[row][col] = 'v',
            'v' => self.matrix[row][col] = '<',
            '<' => self.matrix[row][col] = '^',
            _ => (),
        }
    }

    pub fn move_up(&mut self, mut row: usize, mut col: usize) -> bool {
        while row < self.matrix.len() && col < self.matrix[row].len() {
            match self.matrix[row][col] {
                '^' => {
                    if row == 0 {
                        return true;
                    }

                    row -= 1;

                    if self.matrix[row][col] == '#' {
                        self.rotate_gaurd(row + 1, col);
                        row += 1;
                        continue;
                    }

                    self.matrix[row + 1][col] = 'X';
                    self.matrix[row][col] = '^';
                }
                '>' => {
                    if col == self.matrix[row].len() - 1 {
                        return true;
                    }

                    col += 1;

                    if self.matrix[row][col] == '#' {
                        self.rotate_gaurd(row, col - 1);
                        col -= 1;
                        continue;
                    }

                    self.matrix[row][col - 1] = 'X';
                    self.matrix[row][col] = '>';
                }
                'v' => {
                    if row == self.matrix.len() - 1 {
                        return true;
                    }

                    row += 1;

                    if self.matrix[row][col] == '#' {
                        self.rotate_gaurd(row - 1, col);
                        row -= 1;
                        continue;
                    }

                    self.matrix[row - 1][col] = 'X';
                    self.matrix[row][col] = 'v';
                }
                '<' => {
                    if col == 0 {
                        return true;
                    }

                    col -= 1;

                    if self.matrix[row][col] == '#' {
                        self.rotate_gaurd(row, col + 1);
                        col += 1;
                        continue;
                    }

                    self.matrix[row][col + 1] = 'X';
                    self.matrix[row][col] = '<';
                }
                _ => {
                    eprintln!("Unexpected char");
                    return false;
                }
            }
        }

        return false;
    }

    pub fn count_x(&self) -> usize {
        let mut total: usize = 0;

        for row in &self.matrix {
            for col in row {
                if *col == 'X' {
                    total += 1;
                }
            }
        }

        return total;
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut matrix: String = String::new();

        for row in &self.matrix {
            matrix.push_str(&String::from_iter(row));
            matrix.push_str("\n");
        }

        write!(f, "{}", matrix)
    }
}
