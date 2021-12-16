use anyhow::Result;
use std::fs::read_to_string;

fn grow_lanterns(fishes: &mut Vec<u8>) {
    let mut new_fishes: Vec<u8> = Vec::new();
    for fish in fishes.iter_mut() {
        if 0 == *fish {
            new_fishes.push(8);
            *fish = 6;
        } else {
            *fish -= 1;
        }
    }

    fishes.append(&mut new_fishes);

}

fn main() -> Result<()> {
    let mut file_data = read_to_string("test_input")?;

    file_data.retain(|c| c.is_digit(10) || c == ',' );

    let mut fish_vec: Vec<u8> = file_data
        .split(',')
        .filter_map(|v| {
            v.parse::<u8>().ok()
        })
        .collect();

    
    println!("Initial state: {:?}", &fish_vec);
    for i in 0..80 {
        grow_lanterns(&mut fish_vec);
        println!("day: {:3}  amt: {:7}", i, fish_vec.len());
    }


    Ok(())
}
