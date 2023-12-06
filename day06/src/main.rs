use std::{fs::File, io::BufReader, io::BufRead, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let mut reader = BufReader::new(file);

    let mut times_str = String::new();
    let _ = reader.read_line(&mut times_str);
    let time = &times_str
        .split_whitespace()
        .collect::<Vec<&str>>()[1..]
        .iter()
        .fold("".to_owned(), |mut concatted, time| {
            concatted.push_str(time);
            concatted
        })
        .parse()
        .unwrap_or(0);
    
    let mut distances_str = String::new();
    let _ = reader.read_line(&mut distances_str);
    let distance = &distances_str
        .split_whitespace()
        .collect::<Vec<&str>>()[1..]
        .iter()
        .fold("".to_owned(), |mut concatted, distance| {
            concatted.push_str(distance);
            concatted
        })
        .parse()
        .unwrap_or(0);

    let mut result: i64 = 1;

    for t in 1..=*time {
        let d = t * (time - t);
        if d > *distance {
            result *= time - t - t + 1;
            break;
        }
    }

    println!("{result}");

    Ok(())
}
