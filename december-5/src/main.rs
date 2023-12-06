use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

struct MapEntry{
    destination_range_start : i32,
    source_range_start : i32,
    range_size : i32
}

fn main() {
    let map_entries_str = read_file("input.txt");
    let map_entries = parse_map_entries(&map_entries_str);
    let map = initialize_custom_map(&map_entries);
    let custom_map = fill_default_keys(&map);

    let mut keys : Vec<i32>= custom_map.keys().cloned().collect();

    keys.sort();

    for i in 0..keys.len(){
        println!("{} to {} ", keys[i], custom_map[&keys[i]]);
    }
}

fn initialize_custom_map(entries : &Vec<MapEntry>) -> HashMap<i32, i32>{
    let mut custom_map = HashMap::new();

    for entry in entries{
        for i in 0..entry.range_size{
            custom_map.insert(entry.source_range_start + i, entry.destination_range_start + i);
        }
    }

    return custom_map;
}

fn parse_map_entries(entries_str : &String) -> Vec<MapEntry>{
    let mut maps_entry = Vec::new();

    for line in entries_str.lines() {
        let numbers_str : Vec<String> = line.split_whitespace().into_iter().map(String::from).collect();
        let numbers : Vec<i32> = numbers_str.iter().map(|s| s.parse::<i32>().expect("Conversion Failed")).collect();
        assert_eq!(numbers.len(),3);
        maps_entry.push(MapEntry { destination_range_start: numbers[0], source_range_start: numbers[1], range_size:numbers[2]})
    }

    return maps_entry;
}

fn fill_default_keys(input_map : &HashMap<i32, i32>) -> HashMap<i32, i32>{
    let mut output_map = input_map.clone();

    for key in 0..100{
        if input_map.contains_key(&key){
            continue;
        }
        output_map.insert(key,key);
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