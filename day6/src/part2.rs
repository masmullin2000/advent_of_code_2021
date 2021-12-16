use anyhow::Result;
use std::fs::read_to_string;

fn grow_lanterns(fishes: &mut [u64; 9]) {
    let zeros = fishes[0];

    for i in 1..fishes.len() {
        fishes[i-1] = fishes[i];
    }

    fishes[6] += zeros;
    fishes[8] = zeros;
}

fn count_fishes(fishes: &[u64; 9]) -> u64 {
    let mut rc = 0u64;
    for fish in fishes {
        rc += *fish as u64;
    }

    rc
}

fn main() -> Result<()> {
    let mut file_data = read_to_string("input")?;

    file_data.retain(|c| c.is_digit(10) || c == ',' );

    let fish_vec: Vec<usize> = file_data
        .split(',')
        .filter_map(|v| {
            v.parse::<usize>().ok()
        })
        .collect();

    let mut fish_arr = <[u64; 9]>::default();

    for fish in fish_vec {
        fish_arr[fish] += 1;
    }
    
    //println!("Initial state: {:?}", &fish_vec);
    for _ in 0..256 {
        grow_lanterns(&mut fish_arr);
    }

    println!("there are {} fishes", count_fishes(&fish_arr));

    Ok(())
}
