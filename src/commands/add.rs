use std::{collections::HashMap, env, fs};

use crate::utils::{changes::Status, create_file::create_file, hash_map_insert::hash_map_insert};



pub fn add(){

    let mut add_status:Status = Status::new();
    let current_dir = env::current_dir().unwrap();
    let add_hash:&HashMap<String,String> = add_status.calculate_hash(&current_dir);

     match create_file("add_hash.scr".to_string()){
        Ok(file) => {
            hash_map_insert(add_hash, file).unwrap();
        }
        Err(e) => panic!("{}",e)
     }



}

