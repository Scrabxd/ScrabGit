use std::{collections::HashMap, fs::File, io::Write};

pub fn hash_map_insert(hash:&HashMap<String,String> , mut file: File) -> Result<(),std::io::Error> {
            match file.write_all(serde_json::to_string(hash).expect("Error serializing HashMap").as_bytes()){
                Ok(_) => {
                    Ok(())
                }
                Err(e) => {
                    panic!("{}",e)
                }
            }
}