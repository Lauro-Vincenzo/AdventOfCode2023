use std::fs::File;
use std::io::Read;

fn main() {
    let calibration_document = read_file("input.txt");

    let mut sum : i32 = 0;

    for line in calibration_document.lines(){
        let first_char_digit = find_first_digit(&line);
        let second_char_digit = find_second_digit(&line);
        let final_digit = first_char_digit.to_string() + &second_char_digit.to_string();
        let digit = final_digit.parse::<i32>().expect("Conversion Failed!");
        sum += digit;
    }

    println!("Sum is: {sum}")
}

fn find_first_digit(string_to_parse : &str) -> char{
    let char_digit_result = string_to_parse.chars().find(|c| c.is_digit(10));
    
    let char_digit = match char_digit_result {
        Some(value) => value,
        None => '.'
    };

    return char_digit;
}

fn find_second_digit(string_to_parse : &str) -> char{
    let char_digit_result = string_to_parse.chars().rev().find(|c| c.is_digit(10));
    
    let char_digit = match char_digit_result {
        Some(value) => value,
        None => '.'
    };

    return char_digit;
}

fn read_file(path : &str) -> String{
    let mut file = File::open(path).expect("Unable to find file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).expect("Unable to read from file");
    return file_content;
}
