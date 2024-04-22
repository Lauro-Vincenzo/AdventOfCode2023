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
    let mut routes : HashMap<String, Route> = HashMap::new();
    for (index, route_str) in routes_str.iter().enumerate() {
        if route_str.is_empty(){
            continue;
        }

        // Debug
        if (index > 1){
            continue;
        }

        println!("Debug: Element at position {}: {:?}", &index, &route_str);

        let mut split_route : Vec<String> = route_str.split_whitespace().map(String::from).collect();
        split_route.retain(|element| !element.is_empty());
        let split_route = split_route;
        //assert!(split_route.len() == 3, "Route string is not well formatted");

        //routes.insert(split_route[0].clone(), Route{leftEnd: split_route[1].clone(), rightEnd: split_route[2].clone()});

        for element in split_route.iter(){
            let clean_split_element = strman::remove_blacklist_chars(element.to_string(), vec!['(', ')', ',', '=', ' ']);
            println!("Debug: {:?}", clean_split_element);
        }

        // Parse the route string

    }

    todo!()
}