use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, prelude::*};

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let mut reader = BufReader::new(file);
    
    let steps: Vec<char>;
    {
        let mut steps_str = String::new();
        let _ = reader.read_line(&mut steps_str);
        steps = steps_str.trim_end().chars().collect();

        let mut trash = String::new();
        let _ = reader.read_line(&mut trash);
    }
    
    let mut node_list: HashMap<String, Node> = HashMap::new();

    for line_res in reader.lines() {
        let line = line_res?;
        let line_inp = line.split(" = ").collect::<Vec<&str>>();

        let node_str = line_inp[1].split(", ").collect::<Vec<&str>>();
        let left = node_str[0][1..].to_owned();
        let right = node_str[1][0..3].to_owned();

        node_list.insert(line_inp[0].to_owned(), Node{left, right});
    }

    let mut step_counts: Vec<i64> = Vec::new();
    let starts = node_list.keys().filter(|k| k.chars().nth(2).unwrap() == 'A').collect::<Vec<&String>>();

    for start in starts {
        let mut step_count = 0;
        let mut current = start;
        loop {
            let inst_idx = step_count % steps.len();
            let start = node_list.get(current).unwrap();

            let inst = steps[inst_idx];
            match inst {
                'L' => { current = &start.left; },
                'R' => { current = &start.right; },
                _ => {},
            }

            if current.chars().nth_back(0).unwrap() == 'Z' {
                break;
            }

            step_count += 1;
        }
        step_count += 1;
        step_counts.push(step_count as i64);
    }

    let result = step_counts.into_iter().reduce(|acc, n| lcm(acc, n)).unwrap();

    println!("{result}");

    Ok(())
}

fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 {
        return b
    }
    gcd(b % a, a)
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}
