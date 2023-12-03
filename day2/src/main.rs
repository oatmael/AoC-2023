use std::{
    cmp::Ordering,
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::Path,
};

fn main() {
    print!("Specify path to input: ");
    io::stdout().flush().expect("Failed to flush");

    let mut input_file: String = String::new();

    io::stdin()
        .read_line(&mut input_file)
        .expect("Failed to read line");

    let input_file = Path::new(input_file.trim());
    let input_file: File = match File::open(input_file) {
        Err(why) => panic!("Couldn't open {}: {}", input_file.display(), why),
        Ok(file) => file,
    };

    let reader = BufReader::new(input_file);

    let mut number = 0;
    for line in reader.lines() {
        match read_line(line.unwrap()) {
            Some(index) => number += index,
            None => (),
        }
    }

    println!("Result: {number}");
}

#[derive(PartialEq, Eq)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        return Self {
            red: red,
            green: green,
            blue: blue,
        };
    }
}

impl Ord for Round {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.green > other.green || self.red > other.red || self.blue > other.blue {
            return Ordering::Greater;
        } else if self.green == other.green && self.red == other.red && self.blue == other.blue {
            return Ordering::Equal;
        }

        return Ordering::Less;
    }
}

impl PartialOrd for Round {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

const KNOWN_AMOUNTS: Round = Round {
    red: 12,
    green: 13,
    blue: 14,
};

fn read_line(line: String) -> Option<u32> {
    let line = line;
    let mut rounds: Vec<Round> = Vec::new();

    let mut index = 0;
    let game_number: Vec<&str> = line.split(":").collect();
    if let Some(game_number) = game_number.get(0) {
        let game_number: Vec<&str> = game_number.split_whitespace().collect();
        if let Some(game_number) = game_number.get(1) {
            index = game_number.parse().unwrap();
        } else {
            return None;
        }
    }

    let round_line = game_number.get(1)?;

    for round in round_line.split(";") {
        let mut game = Round::new(0, 0, 0);

        for color in round.split(",") {
            let parts: Vec<&str> = color.split_whitespace().collect();

            match parts.get(1) {
                Some(value) => {
                    let amount = parts.get(0).unwrap().parse().unwrap();

                    match *value {
                        "red" => game.red = amount,
                        "blue" => game.blue = amount,
                        "green" => game.green = amount,
                        _ => (),
                    }
                }
                None => (),
            }
        }

        rounds.push(game);
    }

    let invalid_rounds: Vec<&Round> = rounds
        .iter()
        .filter(|round| *round > &KNOWN_AMOUNTS)
        .collect();

    if invalid_rounds.len() > 0 {
        return None;
    }

    return Some(index);
}
