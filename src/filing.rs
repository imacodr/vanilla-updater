use std::fs;
use std::io;
use std::io::*;

use colored::*;

pub fn write_vanilla(vanilla_color: String, vanilla_style: String, path: String) {
    match fs::copy("./assets/ColorfulGraphite.png", format!("{}/ClassIcons.png", path)) {
        Err(er) => panic!("{}", er),
        Ok(_) => println!("{}", "Successfully installed Vanilla 💚".green())
    };

}
