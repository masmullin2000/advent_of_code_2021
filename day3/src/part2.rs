use anyhow::Result;
use std::fs::read;

const LINE_FEED: u8 = 0x0A;
const ZERO: u8 = 48;
const ONE: u8 = 49;

fn reduce(mut numbers: Vec<Vec<u8>>, idx: usize, kmc: bool) -> Vec<u8> {

    if numbers.len() == 1 {
        return numbers.remove(0)
    }

    let mut mc = 0i8;

    for num in &numbers {
        if num[idx] == 0 {
            mc -= 1;
        } else {
            mc += 1;
        }
    }

    let mc = if mc >= 0 {
        1u8
    } else {
        0
    };

    if kmc {
        numbers.retain(|n| n[idx] == mc);
    } else {
        numbers.retain(|n| n[idx] != mc); 
    }

    reduce(numbers, idx+1, kmc)

}

fn main() -> Result<()> {
    let file_data = read("input")?;

    let mut num_idx = 0;
    let mut numbers: Vec<Vec<u8>> = vec![Vec::new()];

    for byte in file_data {
        if byte == LINE_FEED {
            num_idx += 1;
            numbers.push(Vec::new());
            continue;
        }

        if ZERO == byte {
            numbers[num_idx].push(0);
        } else if ONE == byte {
            numbers[num_idx].push(1);
        } else {
            panic!("error");
        }
    }
    numbers.pop();

    let oxy = numbers.clone();
    let co2 = numbers.clone();

    let mut oxygen = format!("{:?}", reduce(oxy, 0, true));
    oxygen.retain(|n| n == '1' || n == '0');

    let mut carbon = format!("{:?}", reduce(co2, 0, false));
    carbon.retain(|n| n == '1' || n == '0');


    let o = i32::from_str_radix(&oxygen, 2)?;
    let c = i32::from_str_radix(&carbon, 2)?;

    println!("{} :: {}", oxygen, o);
    println!("{} :: {}", carbon, c);
    println!("life support {}", o * c);

    Ok(())
}
