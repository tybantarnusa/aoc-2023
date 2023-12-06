use std::{fs::File, io::BufReader, io::BufRead, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let mut reader = BufReader::new(file);

    let mut times_str = String::new();
    let _ = reader.read_line(&mut times_str);
    let times = &times_str
        .split_whitespace()
        .collect::<Vec<&str>>()[1..]
        .iter()
        .map(|t| t.parse().unwrap_or(0))
        .collect::<Vec<i32>>();
    
    let mut distances_str = String::new();
    let _ = reader.read_line(&mut distances_str);
    let distances = &distances_str
        .split_whitespace()
        .collect::<Vec<&str>>()[1..]
        .iter()
        .map(|t| t.parse().unwrap_or(0))
        .collect::<Vec<i32>>();

    let mut result = 1;

    for (i, time) in times.iter().enumerate() {
        let distance = distances[i];
        for t in 1..=*time {
            let d = t * (time - t);
            if d > distance {
                result *= time - t - t + 1;
                break;
            }
        }
    }

    println!("{result}");

    Ok(())
}
