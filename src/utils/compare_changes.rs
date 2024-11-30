
use core::panic;
use std::{collections::HashMap, env};

use colored::Colorize;

use super::{changes::Status, print_changes::print_changes};


    // hash_str comes populated from the hashes_current state as a string.
    // New changes will come populated with a snapshot of the state of the program currently.


    fn compare_changes(hash_str: String, new_changes: &HashMap<String, String>, print_mode: String) -> HashMap<String, String> {
        let mut changes: HashMap<String, String> = HashMap::new();
    
        let current_hashes: HashMap<String, String> = match serde_json::from_str(&hash_str) {
            Ok(hash_map) => hash_map,
            Err(e) => panic!("Failed to parse JSON: {}", e),
        };
    
        changes.insert("print_mode".to_string(), print_mode);
    
        for (key, value) in &current_hashes {
            if let Some(new_value) = new_changes.get(key) {
                if value != new_value {
                    changes.insert(format!("Modified: {}", key), "".to_string());
                }
            } else {
                changes.insert(format!("Deleted: {}", key), "".to_string()); 
            }
        }
    
        for (key, _new_value) in new_changes {
            if !current_hashes.contains_key(key) {
                changes.insert(format!("New: {}", key), "".to_string());
            }
        }
    
        if new_changes.len() == current_hashes.len() {
            for (key, _new_value) in new_changes {
                if !current_hashes.contains_key(key) {
                    changes.insert(format!("Rename: {}", key), "".to_string()); 
                }
            }
        }    

        return changes;

    }


//Data will contain a String which is a hash made string of the current State of the project and a hashmap of the current state in this way
/*
    {
        print_mode: This will tell the program what color to print it.
        hash_str_add: This one has the add parameters
        hash_str_curr: This one has the current state of the program.
    }
*/


pub fn read_changes(data:HashMap<String,String>){

    let mut status_current = Status::new();

    let mut printer:Vec<HashMap<String,String>> = Vec::new();

    let current_dir = env::current_dir().unwrap();

    let changes = status_current.calculate_hash(&current_dir);
            
    
    for(key,value) in data{
            match key.as_str() {
                "hash_str_add" => {
                    printer.push(compare_changes(value, changes,"added".to_string()));
                }
                "hash_str_curr" => {
                    printer.push(compare_changes(value, changes,"prior".to_string()));
                }

                _ =>{
                    panic!("Other key {}, with Value: {} Was found, {}.", key,value,"fatal_error".bright_red().to_string())
                }
            
            }
            
        }
        print_changes(printer);
    }



