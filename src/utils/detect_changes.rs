use core::panic;
use std::{collections::HashMap, env};
use std::fs;
use std::path::Path;
use regex::Regex;
use walkdir::WalkDir;
use sha2::{Sha256, Digest};

use crate::helpers::read_ignore::{read_ignore, should_ignore};
// Struct used for the creation of hashtable and other functions.
pub struct Status {
    file_hash: HashMap<String,String>
}


impl Status{
    pub fn new() -> Self{
        Status {
            file_hash: HashMap::new(),
        }
    }
    pub fn add_file(&mut self, file_name: String, hash:String){
        self.file_hash.insert(file_name, hash);
    }

    pub fn calculate_hash(&mut self, path: &Path) -> &HashMap<String,String>{

        let ignore:Vec<Regex> = read_ignore();

        for entry in WalkDir::new(path).into_iter().filter_map(Result::ok){

            let file_path:&Path = entry.path();
            let condition:&String = &file_path.to_string_lossy().to_string();


            if file_path.is_dir() && file_path.file_name().unwrap().to_str().unwrap().starts_with('.') {
                continue;
            }
            if should_ignore(&condition, &ignore){
                continue
            }
            if entry.file_type().is_file(){                    
                    match fs::read(file_path){
                        Ok(contents)=>{

                            //Hashing
                            let mut hasher = Sha256::new();
                            hasher.update(&contents);
                            let hash_result = hasher.finalize();

                            let hash_str = format!("{:x}",hash_result);
                            self.add_file(file_path.to_string_lossy().to_string(), hash_str);


                        }
                        Err(e) => {
                            panic!("Error while hashing {}",e)
                        }
                    }
                }
            }
            return &self.file_hash;
        }
    
    

        pub fn get_current_hashes(&mut self){
            let dir:&Path = Path::new(".scrab/hashes.scr");
    
            match fs::read_to_string(dir){
                Ok(contents) => {
                    println!("{}", contents)
                }
                Err(e)=>{
                    eprintln!("{}",e)
                }
            }        
        }
}


pub fn detect_changes(){

    let current_dir = env::current_dir().unwrap();

    let mut status = Status::new();

    status.calculate_hash(&current_dir);
    
    // status.get_current_hashes();


}