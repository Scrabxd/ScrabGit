use core::panic;
use std::io::{self, ErrorKind};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use regex::Regex;
use walkdir::WalkDir;
use sha2::{Sha256, Digest};

use crate::helpers::read_ignore::{read_ignore, should_ignore};
// Struct used for the creation of hashtable and other functions.
pub struct Status {
    file_hash: HashMap<String ,String>,
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
            let condition:&str = &file_path.to_string_lossy().to_string();


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
    
    

        pub fn get_current_hashes(&mut self, hash_path:&Path) -> Result<String , io::Error>{
            
            if hash_path.exists(){   
                match fs::read_to_string(hash_path) {
                    Ok(contents) => return Ok(contents),
                    Err(e) => return Err(e),
                }
            }else{
                Err(io::Error::new(ErrorKind::NotFound, "Hashes couldn't be found please re-run the init command."))
            }


    }
}
