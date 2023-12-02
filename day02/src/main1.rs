use std::{fs::File, error::Error, io::{self, BufRead}, collections::HashMap};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = io::BufReader::new(file);
    
    let mut sum = 0;

    let possibles = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    'game: for line_res in reader.lines() {
        let line = line_res?;
        let record: Vec<&str> = line.split(": ").collect();
        let game_num_str = record[0].split(" ").collect::<Vec<&str>>()[1]; 
        let game_num = game_num_str.parse::<i32>()?;
        
        let set = record[1].split("; ").collect::<Vec<&str>>();
        for s in set {
            let cube = s.split(", ").collect::<Vec<&str>>();

            for c in cube {
                let cube_set = c.split(" ").collect::<Vec<&str>>();
                let cube_num = cube_set[0].parse::<i32>()?;

                if cube_num > *possibles.get(cube_set[1]).unwrap_or(&0) {
                    continue 'game;
                }
            }
        }

        sum += game_num;
    }

    println!("{sum}");

    Ok(())
}
