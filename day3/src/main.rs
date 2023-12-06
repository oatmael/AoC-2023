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

    let mut grid = Grid::new();
    for line in reader.lines() {
        match line {
            Ok(row) => grid.add_row(Row::string_to_row(row)),
            Err(_) => (),
        }
    }

    println!("Result: {}", grid.sum_coords());
}

struct Grid {
    rows: Vec<Row>,
    numbers: Vec<GridNumber>,
}

#[derive(Debug)]
struct Row {
    cells: Vec<char>,
}

enum GridBounds {
    X,
    Y,
}

#[derive(Debug)]
struct GridNumber {
    value: u32,
    y: usize,
    start_x: usize,
    end_x: usize,
}

impl Grid {
    fn new() -> Self {
        return Grid {
            rows: Vec::new(),
            numbers: Vec::new(),
        };
    }

    fn get_coord(&self, x: usize, y: usize) -> Result<&char, GridBounds> {
        match self.rows.get(y) {
            Some(row) => match row.cells.get(x) {
                Some(cell) => return Ok(cell),
                None => return Err(GridBounds::X),
            },
            None => return Err(GridBounds::Y),
        }
    }

    fn get_surrounding_symbols(&self, number: &GridNumber) -> Vec<char> {
        let mut symbols: Vec<char> = Vec::new();

        let number_range = number.start_x..number.end_x + 1;

        'y: for y in 0.max(number.y as i32 - 1) as usize..(number.y + 2) {
            'x: for x in 0.max(number.start_x as i32 - 1) as usize..(number.end_x + 2) {
                if y == number.y && number_range.contains(&x) {
                    continue;
                }

                let value = match self.get_coord(x, y) {
                    Ok(cell) => cell,
                    Err(err) => match err {
                        GridBounds::X => break 'x,
                        GridBounds::Y => break 'y,
                    },
                };

                symbols.push(value.clone());
            }
        }

        return symbols;
    }

    fn add_row(&mut self, row: Row) {
        let y = self.rows.len();

        let mut number: String = String::new();
        let mut start_index = 0;
        for (i, cell) in row.cells.iter().enumerate() {
            if cell.is_ascii_digit() {
                number.push(cell.clone());
                continue;
            }

            if number.len() > 0 {
                self.numbers.push(GridNumber {
                    value: number.parse().unwrap(),
                    y: y,
                    start_x: start_index,
                    end_x: i - 1,
                });

                number = String::new();
            }

            start_index = i + 1;
        }

        self.rows.push(row);
    }

    fn sum_coords(&self) -> u32 {
        let mut sum = 0;

        for number in &self.numbers {
            let surrounding_symbols = self.get_surrounding_symbols(number);

            if surrounding_symbols
                .into_iter()
                .filter(|char| !char.is_ascii_digit() && char != &'.')
                .collect::<Vec<_>>()
                .len()
                > 0
            {
                sum += number.value;
            }
        }

        return sum;
    }
}

impl Row {
    fn string_to_row(line: String) -> Self {
        return Row {
            cells: line.chars().collect(),
        };
    }
}
