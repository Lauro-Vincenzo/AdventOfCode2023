use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

struct MapEntry {
    destination_range_start: i32,
    source_range_start: i32,
    range_size: i32,
}

struct ConversionMap {
    map_name: String,
    map: HashMap<i32, i32>,
}

fn main() {
    let input_info = read_file("input.txt");

    let seeds = extract_seeds(input_info);
    let map_pile = extract_maps_info(input_info);

    let map_infos = split_maps(&map_pile);
    let maps = parse_maps_info(&map_infos);

    let mut keys: Vec<i32> = maps.keys().cloned().collect();

    keys.sort();

    for i in 0..keys.len() {
        println!("{} to {} ", keys[i], custom_map[&keys[i]]);
    }
}

fn extract_seeds(input_info: &String) -> Vec<i32> {
    let first_line = input_info.lines().next().unwrap_or("");
    let delimiter_index = first_line.find(':').expect("Delimiter not found");
    let seeds_numbers_str: String = first_line[delimiter_index..].to_string();

    let numbers_str: Vec<String> = seeds_numbers_str
        .split_whitespace()
        .into_iter()
        .map(String::from)
        .collect();
    let seeds: Vec<i32> = numbers_str
        .iter()
        .map(|s| s.parse::<i32>().expect("Conversion Failed"))
        .collect();

    return seeds;
}

fn extract_maps_info(input_info: &String) -> String {
    let lines: Vec<&str> = input_info.lines().skip(1).collect();
    let output_info = lines.join("\n");

    return output_info;
}

fn split_maps(map_pile: String) -> Vec<String> {
    let mut map_infos = Vec::new();

    let lines: Vec<String> = map_pile.lines().into_iter().map(String::from).collect();
    let mut new_map_info = String::new();

    for line in lines {
        if line.is_empty() {
            map_infos.push(new_map_info);
            new_map_info.clear();
            continue;
        }

        if !new_map_info.is_empty() {
            new_map_info += "\n";
        }

        new_map_info += &line;
    }

    return map_infos;
}

fn initialize_custom_map(entries: &Vec<MapEntry>) -> HashMap<i32, i32> {
    let mut custom_map = HashMap::new();

    for entry in entries {
        for i in 0..entry.range_size {
            custom_map.insert(
                entry.source_range_start + i,
                entry.destination_range_start + i,
            );
        }
    }

    return custom_map;
}

fn parse_maps_info(map_infos: &Vec<String>) -> Vec<ConversionMap> {
    let mut conversion_maps = Vec::new();

    for entry in map_infos {
        let mut entry_lines = entry.lines();
        let map_name = entry_lines.next().unwrap_or_default().to_string();
        let map_entry: String = entry_lines.collect::<Vec<&str>>().join("\n");

        let map_entries = parse_map_entries(&map_entry);

        let map_custom = initialize_custom_map(&map_entries);
        let map: HashMap<i32, i32> = fill_default_keys(&map_custom);

        conversion_maps.push(ConversionMap { map_name, map });
    }

    return conversion_maps;
}

fn parse_map_entries(entries_str: &String) -> Vec<MapEntry> {
    let mut maps_entry = Vec::new();

    for line in entries_str.lines() {
        let numbers_str: Vec<String> = line
            .split_whitespace()
            .into_iter()
            .map(String::from)
            .collect();
        let numbers: Vec<i32> = numbers_str
            .iter()
            .map(|s| s.parse::<i32>().expect("Conversion Failed"))
            .collect();
        assert_eq!(numbers.len(), 3);
        maps_entry.push(MapEntry {
            destination_range_start: numbers[0],
            source_range_start: numbers[1],
            range_size: numbers[2],
        })
    }

    return maps_entry;
}

fn fill_default_keys(input_map: &HashMap<i32, i32>) -> HashMap<i32, i32> {
    let mut output_map = input_map.clone();

    for key in 0..100 {
        if input_map.contains_key(&key) {
            continue;
        }
        output_map.insert(key, key);
    }

    return output_map;
}

fn read_file(path: &str) -> String {
    let mut file = File::open(path).expect("Unable to find file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("Unable to read from file");
    return file_content;
}
