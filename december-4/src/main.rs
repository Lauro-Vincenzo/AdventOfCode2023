use std::fs::File;
use std::io::Read;

struct ScratchCard {
    your_numbers: Vec<i32>,
    winning_numbers: Vec<i32>,
}

fn main() {
    let scratchcards_entry = read_file("input.txt");
    let scratchcards = build_scratchcards(&scratchcards_entry);
    let mut points = 0;
    for scratchcard in scratchcards {
        points += compute_scratchcard_points(&scratchcard);
    }

    println!("Total points: {points}");
}

fn build_scratchcards(entry: &String) -> Vec<ScratchCard> {
    let mut scratchcards = Vec::new();
    for line in entry.lines() {
        let scratchcard = build_scratchcard_from_line(&line.to_string());
        scratchcards.push(scratchcard);
    }

    return scratchcards;
}

fn build_scratchcard_from_line(line: &String) -> ScratchCard {
    let mut scratch_info = String::new();

    if let Some(index) = line.find(':') {
        scratch_info = line[index + 1..].to_string();
    }

    let delimiter_index = match scratch_info.find('|') {
        Some(index) => index,
        None => panic!("Wrong input data!"),
    };

    let your_numbers_info = scratch_info[0..delimiter_index].to_string();
    let winning_numbers_info = scratch_info[delimiter_index + 1..].to_string();

    let your_numbers = parse_numbers_from_string(&your_numbers_info.to_string());
    let winning_numbers = parse_numbers_from_string(&winning_numbers_info.to_string());

    return ScratchCard {
        your_numbers,
        winning_numbers,
    };
}

fn compute_scratchcard_points(card: &ScratchCard) -> i32 {
    let matches = compute_matches(card);
    let mut points = Default::default();

    if matches > 0 {
        let u_matches = (matches - 1) as u32;
        points += i32::pow(2, u_matches);
    } else {
        points = 0;
    }

    return points;
}

fn compute_matches(card: &ScratchCard) -> i32 {
    let mut matches = 0;
    for your_number in &card.your_numbers {
        if card.winning_numbers.contains(&your_number) {
            matches += 1;
        }
    }
    return matches;
}

fn parse_numbers_from_string(num_string: &String) -> Vec<i32> {
    let numbers = num_string
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    return numbers;
}

fn read_file(path: &str) -> String {
    let mut file = File::open(path).expect("Unable to find file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("Unable to read from file");
    return file_content;
}
