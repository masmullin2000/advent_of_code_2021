
use anyhow::Result;
use std::fs::read_to_string;

enum Value {
    Unset,
    Set(i32),
}

use Value::{Set, Unset};

struct Window {
    values: [Value; 3],
    to_set: usize,
}

impl Window {
    fn get_window_val(&mut self, val: i32) -> Value {
        self.values[self.to_set] = Set(val);
        self.to_set += 1;
        if self.to_set == 3 {
            self.to_set = 0;
        }

        let mut rc = 0;
        for v in &self.values {
            if let Set(v) = v {
                rc += v;
            } else{
                return Value::Unset;
            }
        }

        Set(rc)
    }
}

fn main() -> Result<()> {

    let file_data = read_to_string("input")?;

    let mut window = Window {
        values: [Unset, Unset, Unset],
        to_set: 0,
    };
    let mut prev_val = Unset;
    let mut count = 0;

    for line in file_data.lines() {
        let val: i32 = line.parse()?;

        if let Set(cur) = window.get_window_val(val) {
            if let Set(prv) = prev_val {
                if cur > prv {
                    count += 1;
                }
            }
            prev_val = Set(cur);
        }
    }

    println!("Count is {}", count);

    Ok(())
}
