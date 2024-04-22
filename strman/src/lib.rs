use std::fs::File;
use std::io::Read;

pub fn read_file(path: &str) -> String {
    let mut file = File::open(path).expect("Unable to find file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("Unable to read from file");
    file_content
}

pub fn remove_blacklist_chars(text: String, blacklist : Vec<char>) -> String {
    let mut result = String::new();
    for c in text.chars() {
        if !blacklist.contains(&c) {
            result.push(c);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    

    #[test]
    fn it_works() {
    }
}
