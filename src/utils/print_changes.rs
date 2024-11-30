use std::collections::HashMap;

use colored::Colorize;

pub fn print_changes(hash_to_print: Vec<HashMap<String,String>>){
    
    for hashes in hash_to_print{
        let mut print_color = String::new();

        for (key,_) in &hashes{
            if key == "print_mode"{
                
                print_color = match key.as_str() {
                    "prior" => "blue".to_string(),
                    "added" => "red".to_string(),
                    _ => "white".to_string(),
                }
            }
        }

        for (key,_) in hashes{
            if key == "print_mode".to_string() {
                continue;
            }

            let colored_key = match print_color.as_str() {
                "blue" => key.blue(),
                "red" => key.green(),
                _ => key.white()
            };

            println!("{}", colored_key);
        }
    }
}