use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use lazy_static::lazy_static;


lazy_static!{
    static ref LITERAL_CONVERSION_MAP : HashMap<String, char> = {
        let mut map = HashMap::new();
        map.insert("one".to_string(),'1');
        map.insert("two".to_string(),'2');
        map.insert("three".to_string(),'3');
        map.insert("four".to_string(),'4');
        map.insert("five".to_string(),'5');
        map.insert("six".to_string(),'6');
        map.insert("seven".to_string(),'7');
        map.insert("eight".to_string(),'8');
        map.insert("nine".to_string(),'9');
        map.insert("0".to_string(),'0');
        map.insert("1".to_string(),'1');
        map.insert("2".to_string(),'2');
        map.insert("3".to_string(),'3');
        map.insert("4".to_string(),'4');
        map.insert("5".to_string(),'5');
        map.insert("6".to_string(),'6');
        map.insert("7".to_string(),'7');
        map.insert("8".to_string(),'8');
        map.insert("9".to_string(),'9');
        map
    };
} 


fn main() {
    let calibration_document = read_file("input.txt");

    let mut sum : i32 = 0;

    for line in calibration_document.lines(){
        let first_digit = find_first_digit(&line);
        let second_digit = find_second_digit(&line);
        let final_digit = first_digit.to_string() + &second_digit.to_string();
        let digit = final_digit.parse::<i32>().expect("Conversion Failed!");
        sum += digit;
    }

    println!("Sum is: {sum}")
}

fn find_second_digit(string_to_parse : &str) -> char{
    let literal_numbers = LITERAL_CONVERSION_MAP.keys();

    let mut first_found_index : usize = 0;
    let mut candidate_number = String::new();

    for literal_number in literal_numbers {
        if let Some(position) = string_to_parse.rfind(literal_number){
            if position >= first_found_index {
                first_found_index = position;
                candidate_number = literal_number.to_string();
            }
        }else{
            continue;
        }
    }

    return LITERAL_CONVERSION_MAP[&candidate_number];
}

fn find_first_digit(string_to_parse : &str) -> char{
    let literal_numbers = LITERAL_CONVERSION_MAP.keys();

    let mut first_found_index : usize = string_to_parse.len();
    let mut candidate_number = String::new();

    for literal_number in literal_numbers {
        if let Some(position) = string_to_parse.find(literal_number){
            if position < first_found_index {
                first_found_index = position;
                candidate_number = literal_number.to_string();
            }
        }else{
            continue;
        }
    }

    return LITERAL_CONVERSION_MAP[&candidate_number];
}

fn read_file(path : &str) -> String{
    let mut file = File::open(path).expect("Unable to find file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).expect("Unable to read from file");
    return file_content;
}
