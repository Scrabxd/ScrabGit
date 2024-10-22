use core::panic;
use std::{env, fs::{self, File}, io::Write, path::Path};
use crate::utils::detect_changes::Status;

pub fn scrab_init(){
    let mut status = Status::new();
    let current_dir = env::current_dir().unwrap();

    //Create directory for Scrab repository
    let scrab_dir = current_dir.join(".scrab");
    if scrab_dir.exists(){
        println!("Scrab repository already exits")
    }else{
        fs::create_dir(&scrab_dir).expect("Failed to create file folder for repository.")
    }



    let file_path = scrab_dir.join("hashes.scr");

    //Create hashtable with hashes for each file.
    let status = status.calculate_hash(&current_dir);
    match File::create(file_path) {
        Ok(mut file) =>{
            match file.write_all(serde_json::to_string(status).expect("Error serializing HashMap").as_bytes()){
                Ok(_) => {
                    println!("Initialized Scrab repository.")
                }
                Err(e) => {
                    panic!("Error while Initializing repository: {}", e)
                }
            }
        }
        Err(e) =>{
            panic!("{}",e)
        }
    }
    

}