#[path = "filing.rs"]
mod filing;

use std::fs::File;
use std::path::Path;
use std::io;
use std::io::*;
use std::env;
use filing::write_vanilla;


use os_info;
use home;
use colored::*;

pub fn prompt(prompt: &str) -> std::string::String {
    let mut input = String::new();
    println!("{}", prompt.white().bold());
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    return input
}

fn check_os() -> std::string::String {
    let mut path = String::new();

    let info = os_info::get();
    
    match info.os_type() {
        os_info::Type::Windows => {
            println!("{}", "Running Windows...".green().bold().italic());

            match home::home_dir() {
                Some(dir_path) => {
                    path = format!("{}", dir_path.display());
                },
                None => println!("Impossible to get your home dir!"),
            };    

        },
        os_info::Type::Macos => {
            println!("{}", "Running macOS...".green().bold().italic());
            path = String::from("Hey");
        },
        _ => {
            println!("{}", "Running Linux/Undefined OS...".green().bold().italic());
            let input = prompt("Cannot find path to Roblox Studio. Please enter path to /contents/textures manually:");
            path = String::from(input);
        }
    };

    return path
}

pub fn check_studio() {
    let path = check_os();
    let metadata = Path::new(&path).metadata().expect("failed");

    match metadata.is_dir() {
        true => print!("yep its true"),
        false => print!("nope its false")
    }
    
    println!("{}", "The following questions will ask how you want to customize your Vanilla. These will save for future updates.".yellow());
    let colors_answer = prompt("What color do you want your Vanilla to be? (Platinum, Graphite)");

    let style_answer = prompt("What style do you want your Vanilla to be? (Colorful, Mono)");
    
    write_vanilla(colors_answer, style_answer, path);

    // let mut s = String::new();

    // match file.read_to_string(&mut s) {
    //     Err(re) => panic!("{}", re),
    //     Ok(_) => println!("{}", s)
    // }
}
