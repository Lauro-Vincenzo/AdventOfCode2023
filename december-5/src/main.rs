use std::fs::File;
use std::io::Read;

struct MapRule {
    destination_start: u64,
    source_start: u64,
    range_size: u64,
}

struct ConversionMap {
    map_rules: Vec<MapRule>
}

fn main() {
    let input_info = read_file("input.txt");

    let seeds = extract_seeds(&input_info);
    let map_pile = extract_maps_info(&input_info);

    let map_infos = split_maps(&map_pile);
    let maps = parse_maps_info(&map_infos);

    let mut outputs : Vec<u64> = Vec::new();

    for seed in seeds {
         outputs.push(traverse_maps(seed, &maps));
    }

    outputs.sort();
    println!("{}", outputs[0]);
}

fn traverse_maps(input_value : u64, maps: &Vec<ConversionMap>) -> u64{
    let mut current_value = input_value;

    for map in maps {
        current_value = convert_key(&map, current_value);
    }

    return current_value;
}

fn convert_key(map : &ConversionMap, key : u64) -> u64{
    let mut value = key;
    for rule in &map.map_rules{
        if is_in_range(value, rule.source_start, rule.range_size){
            let diff = value - rule.source_start;
            assert!(diff >= 0);
            value = rule.destination_start+diff;
            break;
        }
    }
    return value;
}

fn is_in_range(value: u64, start: u64, range: u64) -> bool {
   return start <= value && value <= start+range;
}
fn extract_seeds(input_info: &String) -> Vec<u64> {
    let first_line = input_info.lines().next().unwrap_or("");
    let delimiter_index = first_line.find(':').expect("Delimiter not found");
    let seeds_numbers_str: String = first_line[delimiter_index + 1..].to_string();

    let numbers_str: Vec<String> = seeds_numbers_str.trim()
        .split_whitespace()
        .into_iter()
        .map(String::from)
        .collect();
    let mut seeds= Vec::new();

    for entry_index in 0..numbers_str.len(){
        println!("Extracting batch number: {}", entry_index);

        if entry_index == numbers_str.len(){
            break;
        }

        if(entry_index % 2 == 0){
            let start = numbers_str[entry_index].parse::<u64>().expect("Conversion Failed");
            let range =  numbers_str[entry_index+1].parse::<u64>().expect("Conversion Failed");
            for i in 0..range{
                seeds.push(start+i);
            }

        }
        println!("Extraction Ended.");
    }

    return seeds;
}

fn extract_maps_info(input_info: &String) -> String {
    let lines: Vec<&str> = input_info.lines().skip(2).collect();
    let output_info = lines.join("\n");

    return output_info;
}

fn split_maps(map_pile: &String) -> Vec<String> {
    let mut map_infos = Vec::new();

    let lines: Vec<String> = map_pile.lines().into_iter().map(String::from).collect();
    let mut new_map_info = String::new();

    for line in lines {
        if line.is_empty() {
            map_infos.push(new_map_info.clone());
            new_map_info.clear();
            continue;
        }

        if !new_map_info.is_empty() {
            new_map_info += "\n";
        }

        new_map_info += &line;
    }

    if !new_map_info.is_empty() {
        map_infos.push(new_map_info.clone());
    }

    return map_infos;
}

fn parse_maps_info(map_infos: &Vec<String>) -> Vec<ConversionMap> {
    let mut conversion_maps = Vec::new();

    for entry in map_infos {
        let mut entry_lines = entry.lines();
        let _ = entry_lines.next().unwrap_or_default().to_string();
        let map_entry: String = entry_lines.collect::<Vec<&str>>().join("\n");

        let map_entries = parse_map_entries(&map_entry);

        conversion_maps.push(ConversionMap {
            map_rules: map_entries,
        });
    }

    return conversion_maps;
}

fn parse_map_entries(entries_str: &String) -> Vec<MapRule> {
    let mut maps_entry = Vec::new();

    for line in entries_str.lines() {
        let numbers_str: Vec<String> = line
            .split_whitespace()
            .into_iter()
            .map(String::from)
            .collect();
        let numbers: Vec<u64> = numbers_str
            .iter()
            .map(|s| s.parse::<u64>().expect("Conversion Failed"))
            .collect();
        assert_eq!(numbers.len(), 3);
        maps_entry.push(MapRule {
            destination_start: numbers[0],
            source_start: numbers[1],
            range_size: numbers[2],
        })
    }

    return maps_entry;
}

fn read_file(path: &str) -> String {
    let mut file = File::open(path).expect("Unable to find file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("Unable to read from file");
    return file_content;
}
