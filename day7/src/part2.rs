use anyhow::Result;
use std::fs::read_to_string;

fn calc_gas_from_steps(steps: i64) -> i64 {
    (steps * (steps + 1)) / 2
}

fn calc_gas(pos: i64, max_gax: i64, crabs: &[i64]) -> Option<i64> {
    let mut gas = 0;

    for i in 0..crabs.len() as i64 {
        let steps = (pos - i).abs();
        let g = crabs[i as usize] * calc_gas_from_steps(steps); 
        gas += g;
        if gas >= max_gax {
            return None;
        }
    }

    Some(gas)
}

fn main() -> Result<()> {
    let mut file_data = read_to_string("input")?;

    file_data.retain(|c| c.is_digit(10) || c == ',');

    let mut crab_vec: Vec<i64> = file_data
        .split(',')
        .filter_map(|v| v.parse::<i64>().ok())
        .collect();

    crab_vec.sort_unstable();

    let largest = crab_vec[crab_vec.len() - 1] as usize;

    let mut crab_arr = vec![0i64; largest + 1];
    let mut cur_gas = i64::MAX;
    for c in crab_vec {
        crab_arr[c as usize] += 1;
    }

    for i in 0..=largest as i64 {
        if let Some(gas) = calc_gas(i, cur_gas, &crab_arr) {
            cur_gas = gas;
        }
    }

    println!("gas used {}", cur_gas);

    Ok(())
}
