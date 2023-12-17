use std::fs::File;
use std::io::Read;
use std::iter::zip;

struct Race{
    time : i32,
    record : i32
}

fn main() {
    let races = parse_races(read_file("input.txt"));
}

fn parse_races(input : String) -> Vec<Race>{
    let races = Vec::new();

    let mut lines = input.lines();
    let times_entry = lines.next().unwrap();
    let distances_entry = lines.next().unwrap();

    let times = parse_values_in_line(times_entry.to_string());
    let distances = parse_values_in_line(distances_entry.to_string());




    return races;
}

fn create_races(times : Vec<i32>, distances: Vec<i32>) -> Vec<Race>{
    let mut races = Vec::new();

    for (time, record) in zip(times, distances) {
        races.push(Race{
            time,
            record
        });
    }

    return races;
}

fn parse_values_in_line(line : String) -> Vec<i32>{
    let (_, numbers_str) = line.split_once(':').expect("Wrong format");
    let numbers = parse_numbers(numbers_str.trim().to_string());
    return numbers;
}

fn parse_numbers(numbers_str : String) -> Vec<i32>{
    let numbers = numbers_str.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
    return numbers;
}

fn read_file(path: &str) -> String {
    let mut file = File::open(path).expect("Unable to find file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("Unable to read from file");
    return file_content;
}