use std::{
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

    let mut number: u32 = 0;
    for line in reader.lines() {
        if let Ok(line) = line {
            match read_line(line) {
                None => continue,
                Some(value) => number += value,
            }
        }
    }

    println!("Result: {number}");
}

struct NumberPos {
    start: usize,
    value: char,
}

const STRING_NUMBERS: [(&str, char); 9] = [
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9'),
];

fn read_line(line: String) -> Option<u32> {
    let mut numbers: Vec<NumberPos> = Vec::new();
    for (i, char) in line.chars().enumerate() {
        if char.is_ascii_digit() {
            numbers.push(NumberPos {
                start: i,
                value: char,
            });
        }
    }

    let slice = &line[..];
    for (string_number, value) in STRING_NUMBERS {
        let matches: Vec<_> = slice.match_indices(string_number).map(|(i, _)| i).collect();
        for index in matches {
            numbers.push(NumberPos {
                start: index,
                value: value,
            });
        }
    }

    numbers.sort_by(|a, b| a.start.cmp(&b.start));

    if numbers.len() > 0 {
        let numbers: Vec<&NumberPos> = vec![numbers.first()?, numbers.last()?];
        let number: String = numbers.into_iter().map(|number| number.value).collect();
        let number: u32 = number.parse().unwrap();

        return Some(number);
    }

    return None;
}
