use std::collections::HashMap;
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

    let mut total_power = 0;
    for game in &games {
        total_power += compute_set_power(&compute_minimum_set(game));
    }
    println!("{total_power}");

    let constraint_set = initialize_constraints();
    let valid_games = filter_valid_games(games, &constraint_set);
    let score = compute_score(&valid_games);

    println!("{score}");
}

fn compute_minimum_set(game: &Game) -> CubeSet {
    let mut minimum_extractions_map: HashMap<String, i32> = HashMap::new();

    minimum_extractions_map.insert("red".to_string(), 0);
    minimum_extractions_map.insert("blue".to_string(), 0);
    minimum_extractions_map.insert("green".to_string(), 0);

    for set in &game.sets {
        for extraction in &set.extractions {
            let current_number = minimum_extractions_map[&extraction.type_of_cube];
            if extraction.number_of_cubes > current_number {
                if let Some(value) = minimum_extractions_map.get_mut(&extraction.type_of_cube) {
                    *value = extraction.number_of_cubes;
                }
            }
        }
    }

    let mut extractions: Vec<CubeExtraction> = Vec::new();

    for key in minimum_extractions_map.keys() {
        extractions.push(CubeExtraction {
            number_of_cubes: minimum_extractions_map[key],
            type_of_cube: key.to_string(),
        })
    }

    return CubeSet { extractions };
}

fn compute_set_power(set: &CubeSet) -> i32 {
    let mut power = 1;
    for extraction in &set.extractions {
        power *= extraction.number_of_cubes;
    }
    return power;
}

fn build_extraction(number: i32, color: &str) -> CubeExtraction {
    CubeExtraction {
        number_of_cubes: number,
        type_of_cube: color.to_string(),
    }
}

fn compute_score(valid_games: &Vec<Game>) -> i32 {
    let mut total_score = 0;
    for game in valid_games {
        total_score += game.game_id;
    }
    return total_score;
}

fn initialize_constraints() -> CubeSet {
    let mut extractions = Vec::new();
    extractions.push(build_extraction(12, "red"));
    extractions.push(build_extraction(14, "blue"));
    extractions.push(build_extraction(13, "green"));

    return CubeSet { extractions };
}

fn filter_valid_games(games: Vec<Game>, constraint_set: &CubeSet) -> Vec<Game> {
    let mut valid_games: Vec<Game> = Vec::new();

    for game in games {
        let mut b_is_game_valid = true;
        for set in &game.sets {
            for extraction in &set.extractions {
                b_is_game_valid &= is_extraction_valid(&extraction, &constraint_set);
            }
        }
        if b_is_game_valid {
            valid_games.push(game);
        }
    }
    return valid_games;
}

fn is_extraction_valid(extraction: &CubeExtraction, constraint_set: &CubeSet) -> bool {
    let mut b_is_extraction_valid = true;

    for constraint in &constraint_set.extractions {
        if extraction.type_of_cube != constraint.type_of_cube {
            continue;
        }

        if extraction.number_of_cubes > constraint.number_of_cubes {
            b_is_extraction_valid = false;
        }
    }
    return b_is_extraction_valid;
}

fn parse_games(game_lines: &Vec<String>) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();

    for line in game_lines {
        let parsed_id = parse_game_id(&line);

        let cube_sets = parse_cube_sets(&line);

        let game = Game {
            game_id: parsed_id,
            sets: cube_sets,
        };

        games.push(game);
    }

    return games;
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

fn parse_cube_sets(game_entry: &String) -> Vec<CubeSet> {
    let mut cube_sets: Vec<CubeSet> = Vec::new();

    let game_init_char_index = game_entry.find(':').unwrap();
    let (_, cube_sets_substr) = game_entry.split_at(game_init_char_index + 1);

    let cube_set_entries: Vec<String> = cube_sets_substr.split(';').map(String::from).collect();

    for cube_set_entry in cube_set_entries {
        let cube_set = parse_cube_set(&cube_set_entry);
        cube_sets.push(cube_set);
    }

    return cube_sets;
}

fn parse_cube_set(cube_set_entry: &String) -> CubeSet {
    let cube_extractions_data: Vec<String> = cube_set_entry.split(',').map(String::from).collect();
    let mut cubes_extractions: Vec<CubeExtraction> = Vec::new();

    for cube_extraction_data in cube_extractions_data {
        let whitespace_index = cube_extraction_data
            .trim()
            .find(' ')
            .expect("No whitespace found!");

        let (number, color) = cube_extraction_data.trim().split_at(whitespace_index + 1);

        let cube_extraction = CubeExtraction {
            number_of_cubes: number.trim().parse::<i32>().expect("Conversion failed!"),
            type_of_cube: color.to_string(),
        };
        cubes_extractions.push(cube_extraction);
    }

    return CubeSet {
        extractions: cubes_extractions,
    };
}

fn read_file(path: &str) -> String {
    let mut file = File::open(path).expect("Unable to find file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("Unable to read from file");
    return file_content;
}
