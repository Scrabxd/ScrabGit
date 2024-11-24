use std::{env, fs};
use crate::utils::{changes::Status, create_file::create_file, hash_map_insert::hash_map_insert};
pub fn repo_init(){
    let mut status = Status::new();
    let current_dir = env::current_dir().unwrap();

    //Create directory for Scrab repository
    let scrab_dir = current_dir.join(".scrab");
    if scrab_dir.exists(){
        println!("Scrab repository already exits")
    }else{
        fs::create_dir(&scrab_dir).expect("Failed to create file folder for repository.")
    }

    let init_hash = status.calculate_hash(&scrab_dir);


    match create_file("hashes_current.scr".to_string()){
        Ok(file) => {
            hash_map_insert(init_hash, file).unwrap();
            return ();
        }
        Err(e)  => panic!("{}",e)
    }

    

}