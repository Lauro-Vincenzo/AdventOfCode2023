use lazy_static::lazy_static;
use std::fs::File;
use std::io::Read;

lazy_static! {
    static ref SYMBOLS: Vec<String> = {
        let mut symbols = Vec::new();
        symbols.push("@".to_string());
        symbols.push("*".to_string());
        symbols.push("+".to_string());
        symbols.push("#".to_string());
        symbols.push("=".to_string());
        symbols.push("/".to_string());
        symbols.push("$".to_string());
        symbols.push("%".to_string());
        symbols.push("-".to_string());
        symbols.push("&".to_string());
        symbols
    };
    static ref DIGITS: Vec<char> = {
        let mut digits = Vec::new();
        digits.push('0');
        digits.push('1');
        digits.push('2');
        digits.push('3');
        digits.push('4');
        digits.push('5');
        digits.push('6');
        digits.push('7');
        digits.push('8');
        digits.push('9');
        digits
    };
}

struct NumberPositionInfo {
    line: usize,
    first_char_index: usize,
    size: usize,
}

fn main() {
    let engine_schematic = read_file("input.txt");
    let schematic_rows: Vec<String> = engine_schematic.lines().map(String::from).collect();

    let mut numbers: Vec<i32> = Vec::new();

    let mut part_number_sum = 0;
    for row in &schematic_rows {
        let part_numbers_in_line = find_part_numbers_in_line(&row, &schematic_rows);
        numbers.extend(&part_numbers_in_line);
        let part_number_line_sum: i32 = part_numbers_in_line.iter().sum();
        part_number_sum += part_number_line_sum;
    }

    numbers;
    println!("{part_number_sum}");
}

fn find_numbers_in_line(line: &String) -> Vec<String> {
    let mut digits_group: Vec<char> = Vec::new();
    let mut found_numbers: Vec<String> = Vec::new();

    for c in line.chars() {
        if c.is_digit(10) {
            digits_group.push(c);
        } else {
            if !digits_group.is_empty() {
                found_numbers.push(digits_group.iter().cloned().collect())
            }
            digits_group.clear();
        }
    }

    return found_numbers;
}

fn find_part_numbers_in_line(row: &String, schematic: &Vec<String>) -> Vec<i32> {
    let mut numbers_str: Vec<String> = find_numbers_in_line(&row);
    let mut numbers: Vec<i32> = Vec::new();

    let row_number = match schematic.iter().position(|x| x == row) {
        Some(value) => value,
        None => panic!("Row not found!"),
    };

    numbers_str.sort();
    numbers_str.dedup();

    for number in numbers_str {
        let number_first_char_positions: Vec<usize> =
            row.match_indices(&number).map(|(index, _)| index).collect();

        for first_char_position in number_first_char_positions {
            let number_position = NumberPositionInfo {
                line: row_number,
                first_char_index: first_char_position,
                size: number.len(),
            };

            let b_is_part_number = is_part_number(&number_position, &schematic);
            if b_is_part_number {
                numbers.push(number.parse::<i32>().expect("Conversion Failed!"));
            }
        }
    }

    return numbers;
}

fn is_part_number(position: &NumberPositionInfo, schematic: &Vec<String>) -> bool {
    let mut b_is_part_number = is_near_to_symbol_horiz(&position, &schematic);

    b_is_part_number |= is_near_to_symbol_vert(&position, &schematic);
    return b_is_part_number;
}

fn is_near_to_symbol_horiz(position: &NumberPositionInfo, schematic: &Vec<String>) -> bool {
    let line = &schematic[position.line];

    let left_symbol_index = match position.first_char_index.checked_sub(1) {
        Some(value) => value,
        None => usize::MAX,
    };

    let symbol_left = match line.chars().nth(left_symbol_index) {
        Some(value) => value,
        None => '\0',
    };
    let symbol_right = match line.chars().nth(position.first_char_index + position.size) {
        Some(value) => value,
        None => '\0',
    };

    let b_left = SYMBOLS.contains(&symbol_left.to_string());
    let b_right = SYMBOLS.contains(&symbol_right.to_string());

    // println!("left {symbol_left}");
    // println!(" b_left {b_left}");
    // println!("right {symbol_right}");
    // println!("b_right {b_right}");
    return b_left || b_right;
}
fn is_near_to_symbol_vert(position: &NumberPositionInfo, schematic: &Vec<String>) -> bool {
    let mut b_is_near_symbol = false;

    let upper_line_index: usize = match position.line.checked_sub(1) {
        Some(value) => value,
        None => usize::MAX,
    };

    let upper_line = match schematic.get(upper_line_index) {
        Some(string) => string,
        None => "",
    };

    let lower_line = match schematic.get(position.line + 1) {
        Some(string) => string,
        None => "",
    };

    if !upper_line.is_empty() {
        b_is_near_symbol |= contains_symbol_near(&position, &upper_line.to_string());
    }
    if !lower_line.is_empty() {
        b_is_near_symbol |= contains_symbol_near(&position, &lower_line.to_string());
    }

    return b_is_near_symbol;
}

fn contains_symbol_near(position: &NumberPositionInfo, line: &String) -> bool {
    let mut b_is_near_symbol = false;
    let leftward_index = match position.first_char_index.checked_sub(1) {
        Some(value) => value,
        None => 0,
    };

    let rightward_index = position.first_char_index + position.size + 1;

    if leftward_index != usize::MAX {
        let sliced = &line[leftward_index..rightward_index];
        for c in sliced.chars() {
            if SYMBOLS.contains(&c.to_string()) {
                b_is_near_symbol = true;
            }
        }
    }

    return b_is_near_symbol;
}

fn read_file(path: &str) -> String {
    let mut file = File::open(path).expect("Unable to find file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("Unable to read from file");
    return file_content;
}
