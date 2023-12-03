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

    for line in reader.lines() {}
}
