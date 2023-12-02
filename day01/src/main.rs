use std::{fs::File, error::Error, io::{self, BufRead}}; 

struct NumText {
    text: String,
    val: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut calibariton_values = Vec::<i32>::new();

    let num_texts = vec![
        NumText{ text: "one".to_owned(), val: 1 },
        NumText{ text: "two".to_owned(), val: 2 },
        NumText{ text: "three".to_owned(), val: 3 },
        NumText{ text: "four".to_owned(), val: 4 },
        NumText{ text: "five".to_owned(), val: 5 },
        NumText{ text: "six".to_owned(), val: 6 },
        NumText{ text: "seven".to_owned(), val: 7 },
        NumText{ text: "eight".to_owned(), val: 8 },
        NumText{ text: "nine".to_owned(), val: 9 },
        NumText{ text: "1".to_owned(), val: 1 },
        NumText{ text: "2".to_owned(), val: 2 },
        NumText{ text: "3".to_owned(), val: 3 },
        NumText{ text: "4".to_owned(), val: 4 },
        NumText{ text: "5".to_owned(), val: 5 },
        NumText{ text: "6".to_owned(), val: 6 },
        NumText{ text: "7".to_owned(), val: 7 },
        NumText{ text: "8".to_owned(), val: 8 },
        NumText{ text: "9".to_owned(), val: 9 },
    ];

    for line_result in reader.lines() {
        let line = line_result?; 

        let mut left = 0;
        let mut right = 0;

        let mut right_pos = 0;
        let mut left_pos = line.len();
        for nt in &num_texts {
            let pos: Vec<usize> = line.match_indices(&nt.text).map(|(i, _)| i).collect();
            if pos.len() > 0 {
                if pos[0] <= left_pos {
                    left_pos = pos[0];
                    left = nt.val;
                }
                if pos[pos.len()-1] >= right_pos {
                    right_pos = pos[pos.len()-1];
                    right = nt.val;
                }
            }
        }

        let calibration_value = left * 10 + right; 

        calibariton_values.push(calibration_value);
    }

    let sum: i32 = calibariton_values.iter().sum();
    println!("{sum}");
    
    Ok(())
}
