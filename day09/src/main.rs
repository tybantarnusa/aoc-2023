use std::{fs::File, error::Error};
use std::io::{BufReader, prelude::*};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut total = 0;
    for line in reader.lines().map(|l| l.unwrap()) {
        let mut nums = line.split(" ").map(|n| n.parse().unwrap()).collect::<Vec<i32>>();
        total += solve(&mut nums);
    }

    println!("{total}");

    Ok(())
}

fn solve(nums: &mut Vec<i32>) -> i32 {
    nums.reverse();
    let mut next_nums = nums.into_iter().fold((Vec::new(), i32::MIN), |mut nums_n: (Vec<i32>, i32), n| {
        if nums_n.1 == i32::MIN {
            return (nums_n.0, *n);
        }

        nums_n.0.push(nums_n.1 - *n);
        (nums_n.0, *n)
    });

    let mut same_diff = true;
    let mut cur = next_nums.0[0];
    for n in next_nums.0[1..].iter() {
       same_diff = cur == *n;
       if !same_diff {
           break;
       }
       cur = *n;
    }

    if same_diff {
        return nums.last().unwrap() - next_nums.0[0];
    }

    next_nums.0.reverse();
    nums.last().unwrap() - solve(&mut next_nums.0)
}
