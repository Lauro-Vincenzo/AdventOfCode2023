use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

struct MapEntry {
    destination_range_start: u32,
    source_range_start: u32,
    range_size: u32,
}

struct ConversionMap {
    _map_name: String,
    map: HashMap<u32, u32>,
}

fn main() {
    let input_info = read_file("input.txt");

    let seeds = extract_seeds(&input_info);
    let map_pile = extract_maps_info(&input_info);
    //
    let map_infos = split_maps(&map_pile);
    let maps = parse_maps_info(&map_infos);
    //
    // let mut outputs : Vec<u32> = Vec::new();
    //
    // for seed in seeds {
    //     outputs.push(traverse_maps(seed, &maps));
    // }
    //
    // outputs.sort();
    // println!("{}", outputs[0]);
}

fn traverse_maps(input_value : u32, maps: &Vec<ConversionMap>) -> u32{
    let mut current_value = input_value;

    for map in maps {
        current_value = map.map[&current_value];
    }

    return current_value;
}

fn extract_seeds(input_info: &String) -> Vec<u32> {
    let first_line = input_info.lines().next().unwrap_or("");
    let delimiter_index = first_line.find(':').expect("Delimiter not found");
    let seeds_numbers_str: String = first_line[delimiter_index + 1..].to_string();

    let numbers_str: Vec<String> = seeds_numbers_str.trim()
        .split_whitespace()
        .into_iter()
        .map(String::from)
        .collect();
    let seeds: Vec<u32> = numbers_str
        .iter()
        .map(|s| s.parse::<u32>().expect("Conversion Failed"))
        .collect();

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

fn initialize_custom_map(entries: &Vec<MapEntry>) -> HashMap<u32, u32> {
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
        let _map_name = entry_lines.next().unwrap_or_default().to_string();
        let map_entry: String = entry_lines.collect::<Vec<&str>>().join("\n");

        let map_entries = parse_map_entries(&map_entry);

        let map_custom = initialize_custom_map(&map_entries);
        let map: HashMap<u32, u32> = fill_default_keys(&map_custom);

        conversion_maps.push(ConversionMap { _map_name, map });
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
        let numbers: Vec<u32> = numbers_str
            .iter()
            .map(|s| s.parse::<u32>().expect("Conversion Failed"))
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

fn fill_default_keys(input_map: &HashMap<u32, u32>) -> HashMap<u32, u32> {
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
