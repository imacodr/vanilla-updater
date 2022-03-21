use std::fs::File;
use std::io;
use std::io::*;
use std::path::Path;
use std::env;

use os_info;

fn check_os() -> std::string::String {
    let mut path = String::new();

    let info = os_info::get();
    
    match info.os_type() {
        os_info::Type::Windows => {
            println!("Running Windows");
            match env::home_dir() {
                Some(dir_path) => println!("{}", dir_path.display()),
                None => println!("Impossible to get your home dir!"),
            };      
        },
        os_info::Type::Macos => {
            println!("Running macOS");
            path = String::from("Hey");
        },
        _ => {
            println!("Running Linux");
            let mut input = String::new();
            println!("Cannot find path to Roblox Studio. Please enter path manually:");
            io::stdin().read_line(&mut input).expect("error: unable to read user input");
            println!("{}",input);
            path = String::from(input);
        }
    };

    return path
}

pub fn exists() {
    let temp_path = check_os();

    println!("{}", temp_path);

    let path = Path::new("./cool.txt");

    let mut file = match File::open(&path) {
        Err(re) => panic!("{}", re),
        Ok(file) => file,
    };

    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Err(re) => panic!("{}", re),
        Ok(_) => println!("{}", s)
    }
}
