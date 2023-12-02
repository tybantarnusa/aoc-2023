use std::{fs::File, error::Error, io::{self, BufRead}}; 

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut calibariton_values = Vec::<i32>::new();

    for line_result in reader.lines() {
        let mut num_str = String::new();
        let line = line_result?; 

        let mut tmp_num: char = '0';
        
        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                if tmp_num == '0' {
                    num_str.push(c);
                }
                tmp_num = c;
            }

            if i == line.len() - 1 {
                num_str.push(tmp_num);
            }
        }

        let calibration_value = num_str.parse::<i32>()?;
        calibariton_values.push(calibration_value);
    }

    let sum: i32 = calibariton_values.iter().sum();
    println!("{sum}");
    
    Ok(())
}
