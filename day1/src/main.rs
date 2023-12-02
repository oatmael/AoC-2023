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

    let mut numbers: Vec<u32> = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            match read_line(line) {
                None => continue,
                Some(value) => numbers.push(value),
            }
        }
    }

    let number: u32 = numbers.iter().sum();
    println!("Result: {number}");
}

fn read_line(line: String) -> Option<u32> {
    let numbers: Vec<char> = line
        .chars()
        .filter(|char: &char| char.is_numeric())
        .collect();

    if numbers.len() > 0 {
        let numbers: Vec<&char> = vec![numbers.first()?, numbers.last()?];
        let number: String = numbers.into_iter().collect();
        let number: u32 = number.parse().unwrap();

        return Some(number);
    }

    return None;
}
