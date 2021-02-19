use std::process::exit;

use colored::*;

pub fn error(err: String) {
    println!("{} {}", "[ERROR]".black().on_red(), err);
    exit(1);
}
