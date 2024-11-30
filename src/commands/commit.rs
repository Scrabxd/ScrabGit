use std::{fs, io::Write, path::Path};

use crate::utils::{changes::Status, create_file::create_file, hash_map_insert::{self, hash_map_insert}};


pub fn commit(){


    let mut status:Status = Status::new();


    let add_hash = status.get_current_hashes(Path::new(".scrab/add_hash.scr")).unwrap();

    match create_file("hashes_current.scr".to_string()){
        Ok(mut file) => {
            file.write_all(add_hash.as_bytes()).unwrap();
        }
        Err(e) => panic!("{}",e)
     }

    

}