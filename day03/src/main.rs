use std::{fs::File, error::Error, io::BufReader, io::BufRead};

fn main() -> Result<(), Box<dyn Error>>{
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut sum = 0;

    let mut rows = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        rows.push(line);
    }

    for (i_row, row) in rows.iter().enumerate() {
        let mut suspected_number = String::new();
        let mut suspected_idx: isize = -1;
        for (i, ch) in row.chars().enumerate() {
            if ch.is_digit(10) {
                suspected_number.push(ch);
                if suspected_idx == -1 {
                    suspected_idx = i as isize;
                }
            }
            if suspected_number.len() > 0 && (!ch.is_digit(10) || i == row.len() - 1) {
                if check_surround(&rows, i_row, suspected_idx as usize, suspected_number.len()) {
                    sum += suspected_number.parse::<usize>()?;
                }
                suspected_number = String::new();
                suspected_idx = -1;
            }
        }
    }

    println!("{sum}");

    Ok(())
}

fn check_surround(rows: &Vec<String>, row: usize, col: usize, len: usize) -> bool {
    if row != 0 {
        let row_check = row - 1;
        for i in col..col+len {
            let symbol_test = &rows[row_check].chars().collect::<Vec<char>>()[i];
            if !symbol_test.is_digit(10) && *symbol_test != '.' {
                return true;
            }
        }
    }

    if row != rows.len() - 1 {
        let row_check = row + 1;
        for i in col..col+len {
            let symbol_test = &rows[row_check].chars().collect::<Vec<char>>()[i];
            if !symbol_test.is_digit(10) && *symbol_test != '.' {
                return true;
            }
        }
    }

    if col != 0 {
        let col_check = col - 1;
        let start_row = if row != 0 { row - 1 } else { row };
        let end_row = if row != rows.len()-1 { row + 1 } else { row };
        for i in start_row..=end_row {
            let symbol_test = &rows[i].chars().collect::<Vec<char>>()[col_check];
            if !symbol_test.is_digit(10) && *symbol_test != '.' {
                return true;
            }
        }
    }

    if col + len != rows[0].len() {
        let col_check = col + len;
        let start_row = if row != 0 { row - 1 } else { row };
        let end_row = if row != rows.len()-1 { row + 1 } else { row };
        for i in start_row..=end_row {
            let symbol_test = &rows[i].chars().collect::<Vec<char>>()[col_check];
            if !symbol_test.is_digit(10) && *symbol_test != '.' {
                return true;
            }
        }
    }

    return false;
}
        
