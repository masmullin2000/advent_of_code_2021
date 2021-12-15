use anyhow::Result;
use std::fs::read_to_string;
//use std::cmp::Ordering;
//

#[derive(Clone, Copy, Debug)]
enum BoardEntry {
    Unchosen(u8),
    Chosen(u8),
}

impl std::fmt::Display for BoardEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let form = match self {
            Unchosen(e) => format!(" {:02} ", e),
            Chosen(e) => format!("*{:02}*", e),
        };
        write!(f, " {}", form)
    }
}
use BoardEntry::*;

#[derive(Debug)]
struct Board {
    rows: [[BoardEntry; 5]; 5],
    ins_row: usize,
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows {
            for entry in row {
                write!(f, "{}", entry)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Board {
    fn new() -> Self {
        Board {
            rows: [[Unchosen(0); 5]; 5],
            ins_row: 0,
        }
    }

    fn insert_row(&mut self, line: &str) -> Result<()> {
        let nums: Vec<&str> = line.split_whitespace().collect();
        for (col, n) in nums.iter().enumerate() {
            self.rows[self.ins_row][col] = Unchosen(n.parse()?);
        }

        self.ins_row += 1;

        Ok(())
    }

    fn is_set(&self) -> bool {
        self.ins_row == 5
    }

    fn check_row(&self, idx: usize) -> bool {
        if let Some(row) = self.rows.get(idx) {
            for entry in row {
                if let Unchosen(_) = entry {
                    return false;
                }
            }

            true
        } else {
            false
        }
    }

    fn check_col(&self, idx: usize) -> bool {
        for row in self.rows {
            if let Unchosen(_) = row[idx] {
                return false;
            }
        }
        true
    }

    fn stamp(&mut self, val: &str) -> Option<u32> {
        let val: u8 = match val.parse() {
            Ok(v) => v,
            _ => return None,
        };

        let mut found = None;
        for (i, row) in &mut self.rows.iter_mut().enumerate() {
            for (j, entry) in row.iter().enumerate() {
                if let Unchosen(e) = entry {
                    if e == &val {
                        found = Some((i, j));
                        break;
                    }
                }
            }

            if let Some((_, idx)) = found {
                row[idx] = Chosen(val);
                break;
            }
        }

        let win = if let Some((row, col)) = found {
            if self.check_col(col) {
                true
            } else {
                self.check_row(row)
            } 
        } else {
            false
        };

        if win {
            let mut rc = 0u32;
            for row in self.rows {
                for entry in row {
                    if let Unchosen(e) = entry {
                        rc += e as u32;
                    }
                }
            }

            Some(rc * val as u32)
        } else {
            None
        }
    }
}

fn main() -> Result<()> {
    let file_data = read_to_string("input")?;

    let mut lines = file_data.lines();
    let numbers = lines.next().unwrap();

    let mut boards: Vec<Board> = Vec::new();
    let mut board = Board::new();

    for line in lines {
        if line.is_empty() {
            if board.is_set() {
                boards.push(board);
                board = Board::new();
            }
            continue;
        }
        board.insert_row(line)?;
    }

    if board.is_set() {
        boards.push(board);
    }

    let numbers = numbers.split(',');
    for num in numbers {
        for board in &mut boards {
            if let Some(rc) = board.stamp(num) {
                println!("winner: {}\n{}", rc, board);
                return Ok(());
            }
        }
        //std::thread::sleep(std::time::Duration::from_secs(2));
    }

    Ok(())
}
