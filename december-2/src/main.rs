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

    print_games(games);
}

fn print_games(games: Vec<Game>) {
    for game in games {
        print!("Game ");
        print!("{}:", game.game_id);
        print!(" ");

        for set in game.sets {
            for extraction in set.extractions {
                print!(
                    "{} {} ",
                    extraction.number_of_cubes, extraction.type_of_cube
                );
            }
        }
        println!("");
    }
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
