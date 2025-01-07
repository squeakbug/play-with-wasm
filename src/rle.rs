use std::collections::HashSet;

#[derive(Debug)]
pub struct Pattern {
    pub cells: HashSet<(usize, usize)>,
}

impl Pattern {
    pub fn new() -> Self {
        Pattern {
            cells: HashSet::new(),
        }
    }

    pub fn from_rle(rle: &str) -> Self {
        let mut pattern = Pattern::new();
        let mut x = 0;
        let mut y = 0;

        for line in rle.lines() {
            if line.starts_with('#') {
                continue; // Skip comments
            }

            for c in line.chars() {
                match c {
                    'o' => {
                        pattern.cells.insert((x, y));
                        x += 1;
                    }
                    '.' => {
                        x += 1;
                    }
                    '$' => {
                        y += 1;
                        x = 0; // Reset x for the new row
                    }
                    '0'..='9' => {
                        let count = c.to_digit(10).unwrap() as usize;
                        for _ in 0..count {
                            pattern.cells.insert((x, y));
                            x += 1;
                        }
                    }
                    '!' => {
                        break; // End of pattern
                    }
                    _ => {}
                }
            }
        }

        pattern
    }
}