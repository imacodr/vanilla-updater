#[path = "filing.rs"]
mod filing;

use filing::write_vanilla;
use std::fs;
use std::io;
use std::path::Path;

use colored::*;
use home;
use os_info;
use serde::Deserialize;
use toml;

#[derive(Deserialize)]
struct Config {
    config: ConfigInside,
}

#[derive(Deserialize)]
struct ConfigInside {
    color: String,
    style: String,
}

pub fn prompt(prompt: &str) -> std::string::String {
    let mut input = String::new();
    println!("{}", prompt.white().bold());
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    return input;
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
                }
                None => println!("Impossible to get your home dir!"),
            };
            println!("{}", path);
        }
        os_info::Type::Macos => {
            println!("{}", "Running macOS...".green().bold().italic());
            path =
                String::from("/Applications/RobloxStudio.app/Contents/Resources/content/textures/");
        }
        _ => {
            println!(
                "{}",
                format!("Running Linux/Undefined OS...")
                    .green()
                    .bold()
                    .italic()
            );
            let input = prompt("Cannot find path to Roblox Studio. Please enter path to /contents/textures manually:");
            path = String::from(input);
        }
    };
    return path;
}

fn write_without_config(path: String) {
    println!("{}", "The following questions will ask how you want to customize your Vanilla. These will save for future updates.".yellow());
    let colors_answer = prompt("What color do you want your Vanilla to be? (Platinum, Graphite)");

    let style_answer = prompt("What style do you want your Vanilla to be? (Colorful, Mono)");
    write_vanilla(
        colors_answer.replace("\n", ""),
        style_answer.replace("\n", ""),
        path,
    );
}

pub fn check_studio() {
    let return_path = check_os();
    let path = Path::new(&return_path);
    match path.is_dir() {
        true => println!(
            "{}",
            "Studio texture folder found...".bold().italic().green()
        ),
        false => panic!("Textures folder not detected (could not be found.)"),
    }
    let mut config_dir_string = String::new();
    match home::home_dir() {
        Some(dir_path) => {
            config_dir_string = format!("{}/.bonfire-tools/", dir_path.display());
        }
        None => println!("Impossible to get your home dir!"),
    };
    let config_dir = Path::new(&config_dir_string);
    match config_dir.is_dir() {
        true => {
            let config_file = format!("{}/icecream.toml", config_dir.display());

            let config_path = Path::new(&config_file);

            match config_path.is_file() {
                true => {
                    let contents = fs::read_to_string(&config_file)
                        .expect("Something went wrong reading the file");

                    let config: Config = toml::from_str(&contents).unwrap();
                    write_vanilla(config.config.color, config.config.style, return_path)
                }
                false => write_without_config(return_path),
            }
        }
        false => write_without_config(return_path),
    }
}
