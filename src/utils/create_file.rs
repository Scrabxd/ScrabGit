use std::{env, fs::{File, OpenOptions}};

pub fn create_file(file_name: String) -> Result<File,std::io::Error>{

    let current_dir = env::current_dir().unwrap();
    let scrab_dir = current_dir.join(".scrab");

    if !scrab_dir.exists() {
        std::fs::create_dir_all(&scrab_dir)?;
    }

    let file_path = scrab_dir.join(file_name);
    
    if file_path.exists(){
        match  OpenOptions::new().read(true).write(true).open(file_path) {
            Ok(file) => return Ok(file),
            Err(e) => return Err(e)
        }

    }

    match File::create(file_path){
        Ok(file) => return Ok(file),
        Err(e) => return Err(e),
    }


}