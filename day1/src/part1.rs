
use anyhow::Result;
use std::fs::read_to_string;

enum PreviousValue {
    Unset,
    Set(i32),
}

use PreviousValue::{Set, Unset};

fn main() -> Result<()> {

    let file_data = read_to_string("input")?;

    let mut prev_val: PreviousValue = Unset;
    let mut count = 0;

    for (i, line) in file_data.lines().enumerate() {
        let val: i32 = line.parse()?;
        println!("{}: {}", i, val);

        if let Set(prv) = prev_val {
            if val > prv {
                count += 1;
            }
        }

        prev_val = Set(val);

    }

    println!("Count is {}", count);

    Ok(())
}
