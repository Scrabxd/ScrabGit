mod commands;
mod utils;
mod helpers;

use commands::init;
use utils::detect_changes::detect_changes;

fn main() {
    //INIT
    let _ = init::scrab_init();

    // detect_changes();

}



