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
}
