use std::collections::HashMap;

struct Route{
    leftEnd : String,
    rightEnd : String,
}

impl Route{
    fn _choose_direction(&self ,direction: char) -> String{
        assert!(direction == 'R' || direction == 'L');
        if direction == 'R'{
            return String::from(&self.rightEnd);
        }
        else if direction == 'L'{
            return String::from(&self.leftEnd);
        }
        else{
            panic!();
        }
    }
}

fn main() {
    let file_content = strman::read_file("input.txt");
    let map_stash: Vec<String> = file_content.lines().map(String::from).collect();
    
    let (direction_str, routes_str): (Vec<String>, Vec<String>) = map_stash.clone().into_iter().partition(|line : &String| line == &map_stash[0].clone());
    println!("Debug: {:?}", direction_str);

    let _routes = generate_routes(routes_str);
}

fn generate_routes(routes_str : Vec<String>) -> HashMap<String, Route>{
    for (index, route_str) in routes_str.iter().enumerate() {
        println!("Debug: Element at position {}: {:?}", &index, &route_str);
    }

    todo!()
}