use std::{collections::HashMap, env};

use crate::utils::{compare_changes::read_changes, changes::Status};



pub fn status() {

    let mut current_status = Status::new();
    let current_dir = env::current_dir().unwrap();

    let current_hash:String = current_status.get_current_hashes();

    let mut hash_cc:HashMap<String,String> = HashMap::new();




    hash_cc.insert("print_mode".to_string(), "prior".to_string());
    hash_cc.insert("print_mode".to_string(),current_hash);


    read_changes(vec![hash_cc]);
    


}

