use std::{error::Error, fs::File, io::BufReader, io::BufRead};

#[derive(Clone)]
struct Card {
    win_list: Vec<i32>,
    my_list: Vec<i32>,
    total: i32,
}

impl Card {
    fn duplicate(&mut self, total: i32) {
        self.total += total;
    }

    fn total_win(&self) -> i32 {
        let mut count = 0;
        for my in self.my_list.iter() {
            if *my == -1 {
                continue;
            }
            if self.win_list.contains(&my) {
                count += 1;
            }
        }
        count
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut scratchcards: Vec<Card> = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?; 
        let num_lists = line.split(": ").collect::<Vec<&str>>()[1].split(" | ").collect::<Vec<&str>>();
        let win_list = num_lists[0].split(" ").map(|s| s.parse().unwrap_or(-1)).collect::<Vec<i32>>();
        let my_list = num_lists[1].split(" ").map(|s| s.parse().unwrap_or(-1)).collect::<Vec<i32>>();

        scratchcards.push(Card{
            win_list,
            my_list,
            total: 1,
        });
    }

    {
        let scratchcards_copy = scratchcards.clone();
        for (i, card) in scratchcards_copy.iter().enumerate() {
            let wins: usize = card.total_win().try_into()?;
            for n in i+1..=wins+i {
                let total = scratchcards[i].total;
                scratchcards[n].duplicate(total);
            }
        }
    }

    let total_scratchcards = scratchcards.iter().fold(0, |total, c| total + c.total);
    println!("{total_scratchcards}");
    Ok(())
}
