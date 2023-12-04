use std::{error::Error, fs::File, io::BufReader, io::BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line_result in reader.lines() {
        let line = line_result?; 
        let num_lists = line.split(": ").collect::<Vec<&str>>()[1].split(" | ").collect::<Vec<&str>>();
        let win_list = num_lists[0].split(" ").map(|s| s.parse().unwrap_or(-1)).collect::<Vec<i32>>();
        let my_list = num_lists[1].split(" ").map(|s| s.parse().unwrap_or(-1)).collect::<Vec<i32>>();

        let mut score = 0;
        for my in my_list {
            if my == -1 {
                continue;
            }

            if win_list.contains(&my) {
                score = if score == 0 { 1 } else { score * 2 };
            }
        }
        
        sum += score;
    }

    println!("{sum}");
    Ok(())
}
