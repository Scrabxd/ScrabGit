
use core::panic;
use std::{collections::HashMap, env};

use colored::Colorize;
use serde::de::value;
use serde_json::Value;

use super::{changes::Status, print_changes::print_changes};


    // hash_str comes populated from the hashes_current state as a string.
    // New chaanges will come populated with a snapshot of the state of the program currently.
    fn compare_changes(hash_str:String, new_changes: &HashMap<String, String>, print_mode:String) -> HashMap<String,String> {

    let mut changes:HashMap<String,String> = HashMap::new();

    let current_hashes:HashMap<String, String> = match serde_json::from_str(&hash_str){
            Ok(hash_map) => hash_map,
            Err(e) => panic!("Failed to parse JSON: {}",e),
        }; 

    changes.insert("print_mode".to_string(), print_mode);

        for (key,value) in &current_hashes{
            if let Some(new_value) = new_changes.get(key){
                if value != new_value{
                    changes.insert("Modified: ".to_string(), key.to_string());
                }
                continue; //TODO Puede que esto este no funcional porque no se como funcionan bien los coninue en RUST.
            }

            // Check if the object is not contained and the lenght is greater than before, then a new file was created.
            if !new_changes.contains_key(key) && new_changes.len() > current_hashes.len(){
                changes.insert("New: ".to_string(), new_changes.get(key).unwrap().to_string());

                continue; //TODO IGUAL QUE EL DE ARRIBA.
            }

            // Check if the new HashMap has the current key and the lenght is the same then it was renamed.
            if new_changes.contains_key(key) && new_changes.len() == current_hashes.len(){
                changes.insert("Rename: ".to_string(),new_changes.get(key).unwrap().to_string());

                continue; //TODO mismo
            }

            // If the previous validation is false, then we can check if the changes are not contained. then it must have been deleted.
            if !new_changes.contains_key(key){
                changes.insert("Deleted: ".to_string(), key.to_string());

                continue; //TODO IGUAL QUE EL DE ARRIBA.
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


pub fn read_changes(data:Vec<HashMap<String,String>>){

    let mut status_current = Status::new();

    let mut printer:Vec<HashMap<String,String>> = Vec::new();

    let current_dir = env::current_dir().unwrap();

    let changes = status_current.calculate_hash(&current_dir);
    
    
    for comparison in data{
        
        let mut hash_str :String = String::new();
        let mut print_mode: String = String::new();

        for(key,value) in comparison{
            match key.as_str() {
                "hash_str_add" => {
                    hash_str = value;
                    print_mode = "added".to_string()
                }
                "hash_str_curr" => {
                    hash_str = value;
                    print_mode = "prior".to_string()
                }
            _ =>{
                panic!("Other key {}, with Value: {} Was found, {}.", key,value,"fatal_error".bright_red().to_string())
            }

            }
        }
        
        printer.push(compare_changes(hash_str, changes,print_mode));

    }

    print_changes(printer);
}


