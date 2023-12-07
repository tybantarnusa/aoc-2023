use std::{fs::File, error::Error, io::BufReader, io::BufRead, collections::HashMap, cmp::Ordering};

#[derive(Debug)]
struct HandObj {
    hand: String,
    val: i32,
    bid: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?; 
    let reader = BufReader::new(file);

    let mut hand_list: Vec<HandObj> = Vec::new();

    for line_res in reader.lines() {
        let line = line_res?;
        let mut hands: HashMap<char, i32> = HashMap::new();

        let inp: Vec<&str> = line.split(" ").collect();
        let hands_str = inp[0];
        let bid = inp[1].parse()?;

        for card in hands_str.chars() {
            if hands.contains_key(&card) {
                let hand_card = hands.get_mut(&card).unwrap();
                *hand_card += 1;
            } else {
                hands.insert(card, 1);
            }
        }

        hand_list.push(HandObj {
            hand: hands_str.to_owned(),
            val: check_hand(&hands),
            bid
        });
    }

    hand_list.sort_by(|a, b| {
        if a.val != b.val {
            return a.val.cmp(&b.val)
        }

        compare_cards(&a.hand, &b.hand)
    });

    let mut total: i64 = 0;
    for (i, h) in hand_list.iter().enumerate() {
        total += (i + 1) as i64 * h.bid as i64;
    }

    println!("{total}");
    Ok(())
}

fn compare_cards(a: &str, b: &str) -> Ordering {
    for i in 0..a.len() {
        let mut num_char: char = a.chars().collect::<Vec<char>>()[i];
        let num_a = to_num(num_char);

        num_char = b.chars().collect::<Vec<char>>()[i];
        let num_b = to_num(num_char);

        let order = num_a.cmp(&num_b);
        if order == Ordering::Equal {
            continue;
        }

        return order;
    }

    Ordering::Equal
}

fn to_num(c: char) -> i32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => c.to_digit(10).unwrap() as i32,
    }
}

fn check_hand(hand: &HashMap<char, i32>) -> i32 {
    let hand_len = hand.len();

    match hand_len {
        1 => 6,
        2 => {
            for h in hand {
                if *h.1 == 4 || *h.1 == 1 {
                    return 5;
                } else if *h.1 == 3 || *h.1 == 2 {
                    return 4;
                }
            }
            0
        },
        3 => {
            for h in hand {
                if *h.1 == 3 {
                    return 3;
                }
                if *h.1 == 2 {
                    return 2;
                }
            }
            0
        },
        4 => 1,
        5 => 0,
        _ => 0,
    }
}
