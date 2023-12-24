use std::fs::File;
use std::io::Read;
use std::iter::zip;
use std::ops::Add;

struct Race{
    time : u64,
    record : u64
}

fn main() {
    let races = parse_races(&read_file("input.txt"));
    let compound_race = parse_compound_race(&read_file("input.txt"));

    let mut combinatorial_scores = 1;
    let mut compound_score = 1;

    for race in &races{
        combinatorial_scores *= find_ways_of_winning(race);
    }

    println!("Multiple races output: {}", combinatorial_scores);
    println!("Single race output: {}", find_ways_of_winning(&compound_race));
}

fn parse_races(input : &String) -> Vec<Race>{
    let mut lines = input.lines();
    let times_entry = lines.next().unwrap();
    let distances_entry = lines.next().unwrap();

    let times = parse_values_in_line(&times_entry.to_string());
    let distances = parse_values_in_line(&distances_entry.to_string());

    let races = create_races(times, distances);
    return races;
}

fn find_ways_of_winning(race : &Race) -> u64{
    let mut ways_of_winning :u64 = 0;
    for hold_time in 1..race.time{
        let run_time = race.time - hold_time;
        let distance = run_time*hold_time;
        if distance > race.record{
            ways_of_winning += 1;
        }
    }
    return ways_of_winning;
}

fn parse_compound_race(input : &String) -> Race{
    let mut lines = input.lines();
    let time_entry = lines.next().unwrap();
    let distance_entry = lines.next().unwrap();

    let time = concatenate_integers_in_line(&time_entry.to_string());
    let distance = concatenate_integers_in_line(&distance_entry.to_string());
    return Race{time,record : distance};
}

fn concatenate_integers_in_line(line : &String) -> u64{
    let numbers = parse_values_in_line(line);
    let mut string_number = String::new();
    for number in numbers{
        string_number.push_str(number.to_string().as_str());
    }

    return string_number.parse::<u64>().expect("Conversion failed!");
}
fn create_races(times : Vec<u64>, distances: Vec<u64>) -> Vec<Race>{
    let mut races = Vec::new();

    for (time, record) in zip(times, distances) {
        races.push(Race{
            time,
            record
        });
    }

    return races;
}

fn parse_values_in_line(line : &String) -> Vec<u64>{
    let (_, numbers_str) = line.split_once(':').expect("Wrong format");
    let numbers = parse_numbers(numbers_str.trim().to_string());
    return numbers;
}

fn parse_numbers(numbers_str : String) -> Vec<u64>{
    let numbers = numbers_str.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect();
    return numbers;
}

fn read_file(path: &str) -> String {
    let mut file = File::open(path).expect("Unable to find file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("Unable to read from file");
    return file_content;
}