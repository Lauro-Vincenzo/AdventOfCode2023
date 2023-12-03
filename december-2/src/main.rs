use std::fs::File;
use std::io::Read;

struct CubeExtraction {
    number_of_cubes: i32,
    type_of_cube: String,
}

struct CubeSet {
    extractions: Vec<CubeExtraction>,
}

struct Game {
    game_id: i32,
    sets: Vec<CubeSet>,
}

fn main() {
    let game_record = read_file("input.txt");
    let game_entries = game_record.lines().map(String::from).collect();
    let games = parse_games(&game_entries);
}

fn parse_games(game_lines: &Vec<String>) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();

    for line in game_lines {
        let parsed_id = parse_game_id(&line);

        let cube_sets = parse_cube_sets(&line);

        let mut game = Game {
            game_id: parsed_id,
            sets: cube_sets,
        };

        games.push(game);
    }

    return games;
}

fn parse_cube_sets(game_entry: &String) -> Vec<CubeSet> {
    let cube_sets: Vec<CubeSet> = Vec::new();
    return cube_sets;
}

fn parse_game_id(game_entry: &String) -> i32 {
    let game_entry_init = game_entry.split(':').next().unwrap_or("");
    let game_entry_id = game_entry_init
        .split_whitespace()
        .rev()
        .next()
        .unwrap_or("");

    return game_entry_id.parse::<i32>().expect("Conversion Failed!");
}

fn read_file(path: &str) -> String {
    let mut file = File::open(path).expect("Unable to find file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("Unable to read from file");
    return file_content;
}
