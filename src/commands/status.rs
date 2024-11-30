use core::panic;
use std::{collections::HashMap, env, path::Path};

use crate::utils::{compare_changes::read_changes, changes::Status};



pub fn status() {
    // "prior" changes prior to the add.
    // "added" changes done after the add.

    let mut current_status = Status::new();
    
    let mut hash_cc:HashMap<String,String> = HashMap::new();

    match current_status.get_current_hashes(Path::new(".scrab/hashes_current.scr")){
        Ok(current_hash) => {
            hash_cc.insert("hash_str_curr".to_string(),current_hash);
        }

        Err(e) => panic!("{}",e)
    }

    match current_status.get_current_hashes(Path::new(".scrab/add_hash.scr")){
        Ok(add_hash) => {
            hash_cc.insert("hash_str_add".to_string(), add_hash);
        }
        Err(e) => panic!("{}",e)
    }

    read_changes(hash_cc);
    


}

