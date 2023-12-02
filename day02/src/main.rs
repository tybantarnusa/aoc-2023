use std::{fs::File, error::Error, io::{self, BufRead}, collections::HashMap};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = io::BufReader::new(file);
    
    let mut sum = 0;

    for line_res in reader.lines() {
        let mut maxes = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0),
        ]);

        let line = line_res?;
        let record: Vec<&str> = line.split(": ").collect();
        
        let set = record[1].split("; ").collect::<Vec<&str>>();
        for s in set {
            let cube = s.split(", ").collect::<Vec<&str>>();

            for c in cube {
                let cube_set = c.split(" ").collect::<Vec<&str>>();
                let cube_num = cube_set[0].parse::<i32>()?;

                if cube_num > *maxes.get(cube_set[1]).unwrap_or(&0) {
                    *maxes.get_mut(cube_set[1]).unwrap() = cube_num;
                }
            }
        }

        let power: i32 = maxes.iter().map(|(_, v)| v).fold(1, |p, v| p * v);
        sum += power;
    }

    println!("{sum}");

    Ok(())
}
