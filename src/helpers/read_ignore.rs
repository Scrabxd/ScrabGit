use std::{env, fs::File, io::{BufRead, BufReader}, path::Path};

use regex::Regex;

pub fn read_ignore() -> Vec<Regex> {
    let file:File = File::open(Path::new(".scrabignore")).unwrap();
    let scrab_path:&Path = Path::new(".scrab");
    let reader = BufReader::new(file);

    let mut ignore: Vec<Regex> = Vec::new();
    ignore.push(Regex::new(&format!(r"^{}/.*",scrab_path.display())).unwrap());

    for line in reader.lines() {
        match line {
            Ok(content) => {
                let trimmed = content.trim();
                if !trimmed.starts_with("//") && !trimmed.is_empty() {
                    let path_pattern = format!(r"{}{}{}",
                        "^",
                        env::current_dir().
                             unwrap().
                             to_string_lossy().
                             to_string(), 
                        trimmed.replace('/', "\\"));
                        
                    match Regex::new(&path_pattern.trim().replace("\\", "\\\\")){
                        Ok(regexp) => {
                            
                            ignore.push(regexp);
                        } 
                        Err(e) => {
                            panic!("Error reading ignore: {}",e)
                        }                       
                    }
                }
            }
            Err(e) => panic!("Error reading .scrabignore file: {}", e),
        }
    }

    ignore
}


pub fn should_ignore(path: &str, ignore_patters:&[Regex]) -> bool {
    for regex in ignore_patters{
        if regex.is_match(path){
            return true
        }
    }
    false
}

