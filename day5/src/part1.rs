use anyhow::Result;
use std::cmp::{max, min};
use std::fs::read_to_string;

struct Matrix {
    rows: Vec<Vec<i32>>,
    row_sz: usize,
    col_sz: usize,
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for i in row {
                if *i == 0 {
                    write!(f, "<...>")?;
                } else {
                    write!(f, "<{:03}>", i)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Matrix {
    fn new(row: usize, col: usize) -> Self {
        let mut m = Matrix {
            rows: Vec::new(),
            row_sz: 0,
            col_sz: 0,
        };
        m.ensure_size(row, col);
        m
    }

    fn ensure_size(&mut self, row: usize, col: usize) {
        if row > self.row_sz {
            self.row_sz = row;
        }
        if col > self.col_sz {
            self.col_sz = col;
        }

        for i in 0..=self.row_sz {
            if self.rows.len() <= i {
                let new_col: Vec<i32> = vec![0; self.col_sz + 1];
                self.rows.push(new_col);
            } else if self.rows[i].len() <= self.col_sz {
                let app_len = self.col_sz - self.rows[i].len();
                let mut app_col: Vec<i32> = vec![0; app_len + 1];
                self.rows[i].append(&mut app_col);
            }
        }
    }

    fn insert_horizonal_line(&mut self, row: usize, col_s: usize, col_e: usize) {
        let row = &mut self.rows[row];

        let from = min(col_s, col_e);
        let to = max(col_s, col_e);

        for item in row.iter_mut().take(to + 1).skip(from) {
            *item += 1;
        }
    }

    fn insert_virtical_line(&mut self, col: usize, row_s: usize, row_e: usize) {
        let from = min(row_s, row_e);
        let to = max(row_s, row_e);

        for (i, row) in self.rows.iter_mut().enumerate() {
            if i >= from && i <= to {
                row[col] += 1;
            }
        }
    }

    fn insert_line(&mut self, line: &str) -> Result<()> {
        let (col_s, row_s, col_e, row_e) = parse_line(line)?;

        if row_s == row_e {
            self.ensure_size(max(row_s, row_e), max(col_s, col_e));
            self.insert_horizonal_line(row_s, col_s, col_e);
        } else if col_s == col_e {
            self.ensure_size(max(row_s, row_e), max(col_s, col_e));
            self.insert_virtical_line(col_s, row_s, row_e);
        }
        Ok(())
    }

    fn overlap(&self) -> u32 {
        let mut amt = 0;
        for row in &self.rows {
            for entry in row {
                if *entry >= 2 {
                    amt += 1;
                }
            }
        }

        amt
    }
}

fn parse_line(line: &str) -> Result<(usize, usize, usize, usize)> {
    let mut s = line.to_string();
    s.retain(|c| !c.is_whitespace());
    s = s.replace("->", ",");
    //println!("{}", s);

    let ss: Vec<usize> = s.split(',').map(|v| v.parse::<usize>().unwrap()).collect();
    Ok((ss[0], ss[1], ss[2], ss[3]))
}

fn main() -> Result<()> {
    let file_data = read_to_string("input")?;

    let mut m = Matrix::new(0, 0);

    for line in file_data.lines() {
        m.insert_line(line)?;
    }

    println!("overlap: {}", m.overlap());

    Ok(())
}
