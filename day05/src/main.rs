use std::{fs::File, error::Error, io::BufReader, io::BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut seeds: Vec<i64> = Vec::new();
    let mut mapping: Vec<String> = Vec::new();

    let mut lines = reader.lines().map(|s| s.unwrap()).collect::<Vec<String>>();
    lines.push("".to_owned());

    for line in lines {
        if seeds.is_empty() {
            let seed_nums: Vec<i64> = line
                .split(": ")
                .nth(1)
                .unwrap()
                .split(" ")
                .map(|n| n.parse::<i64>()
                .unwrap())
                .collect();
            seeds = seed_nums
        }
        
        if line.is_empty() || !line.chars().nth(0).unwrap().is_ascii_digit() {
            if !mapping.is_empty() {
                'sl: for seed in seeds.iter_mut() {
                    for mapp in mapping.iter() {
                        let data: Vec<i64> = mapp.split(" ").map(|n| n.parse::<i64>().unwrap()).collect();
                        let min = data[1];
                        let max = min + data[2] - 1;
                        if *seed >= min && *seed <= max {
                            *seed = *seed - &min + data[0];
                            continue 'sl;
                        }
                    }
                }
                mapping.clear();
            }
            continue;
        }

        mapping.push(line);
    }
    
    let lowest = seeds.iter().min().unwrap();
    println!("{lowest}");

    Ok(())
}
