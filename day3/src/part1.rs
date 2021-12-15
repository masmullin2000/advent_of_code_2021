use anyhow::Result;
use std::fs::read;
use std::cmp::Ordering;

const LINE_FEED: u8 = 0x0A;
const ZERO: u8 = 48;
const ONE: u8 = 49;

fn main() -> Result<()> {
    let file_data = read("input")?;

    let mut gamma_cnt: Vec<i8> = Vec::new();
    let mut pos = 0;

    for byte in file_data {
        if byte == LINE_FEED {
            pos = 0;
            continue;
        }

        if gamma_cnt.len() <= pos {
            gamma_cnt.push(0);
        }

        if ZERO == byte {
            gamma_cnt[pos] -= 1;
        } else if ONE == byte {
            gamma_cnt[pos] += 1;
        } else {
            panic!("error");
        }
        pos += 1;
    }

    let mut gamma: String = String::new();
    for g in gamma_cnt {
        match g.partial_cmp(&0).unwrap_or(Ordering::Equal) {
            Ordering::Less => gamma.push('1'),
            Ordering::Greater => gamma.push('0'),
            _ => panic!("error 2"),
        }
    }

    let mut eps: String = String::new();
    for g in gamma.chars() {
        if g == '1' {
            eps.push('0');
        } else {
            eps.push('1');
        }
    }

    let g = i32::from_str_radix(&gamma, 2)?;
    let e = i32::from_str_radix(&eps, 2)?;
    let power = g * e;

    println!("gamma {} :: {}", gamma, g);
    println!("eps   {} :: {}", eps, e);
    println!("power {}", power);

    Ok(())
}
