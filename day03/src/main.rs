use std::{fs::File, error::Error, io::BufReader, io::BufRead};

struct Node {
    value: i32,
    left: usize,
    right: usize,
    bottom: usize,
    top: usize,
}

impl Node {
    fn new_num(_rows: &Vec<String>, symbol: String, index: usize, row: usize) -> Node {
        Node {
            value: symbol.clone().parse().unwrap_or(0),
            left: index,
            right: index+symbol.len()-1,
            bottom: row,
            top: row,
        }
    }

    fn new_gear(rows: &Vec<String>, symbol: String, index: usize, row: usize) -> Node {
        Node {
            value: symbol.clone().parse().unwrap_or(0),
            left: if index != 0 { index-1 } else { 0 },
            right: if index+symbol.len() != rows.len() { index+1 } else { rows.len()-1 },
            bottom: if row != rows.len()-1 { row+1 } else { rows.len()-1 },
            top: if row != 0 { row-1 } else { 0 },
        }
    }

    fn collide(&self, other: &Node) -> bool {
        self.left <= other.right && self.right >= other.left && self.top <= other.bottom && self.bottom >= other.top
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut sum = 0;

    let mut rows = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        rows.push(line);
    }

    let mut num_list = Vec::<Node>::new();
    let mut gear_list = Vec::<Node>::new();

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
                num_list.push(Node::new_num(&rows, suspected_number.clone(), suspected_idx as usize, i_row));
                suspected_number = String::new();
                suspected_idx = -1;
            }
            if ch == '*' {
                gear_list.push(Node::new_gear(&rows, "*".to_owned(), i, i_row));
            }
        }
    }

    for gear in gear_list {
        let mut pair = Vec::<&Node>::new();        
        for num in num_list.iter() {
            if num_list.is_empty() {
                break;
            }

            if pair.len() == 2 {
                break;
            }
            
            if num.top > gear.bottom {
                break;
            }

            if gear.collide(&num) {
                pair.push(num);
            }
        }

        if pair.len() == 2 {
            sum += pair[0].value * pair[1].value;
        }
    }

    println!("{sum}");

    Ok(())
}
