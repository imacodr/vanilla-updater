use std::fs;
use std::path::Path;

use colored::*;
use home;

pub fn check_dir(dir: &Path) {
    match dir.is_dir() {
        true => {}
        false => fs::create_dir(dir).expect("Error occured when creating dir"),
    }
}

fn write_config(color: String, style: String) {
    let mut write_dir = String::new();
    match home::home_dir() {
        Some(dir_path) => write_dir = format!("{}/.bonfire-tools", dir_path.display()),
        None => println!("Impossible to get your home dir!"),
    };
    let data = format!("[config]\ncolor = '{}'\nstyle = '{}'", color, style);

    let file_dir = format!("{}/icecream.toml", &write_dir);

    check_dir(Path::new(&write_dir));
    fs::write(Path::new(&file_dir), data).expect("Could not write to file!");
}

pub fn write_vanilla(vanilla_color: String, vanilla_style: String, path: String) {
    let color_slice: &str = &vanilla_color.to_owned()[..];
    let style_slice: &str = &vanilla_style.to_owned()[..];
    let color;
    let style;
    match color_slice {
        "Graphite" => color = String::from("Graphite"),
        "Platinum" => color = String::from("Platinum"),
        _ => panic!("Unkown color for Vanilla. (Valid Colors: Graphite, Platinum)"),
    }
    match style_slice {
        "Colorful" => style = String::from("Colorful"),
        "Mono" => style = String::from("Mono"),
        _ => panic!("Unkown style for Vanilla. (Valid Colors: Mono, Colorful)"),
    }
    let vanilla_chosen = format!("{}{}", style, color);
    let to_path = format!("{}ClassImages.png", path);
    let final_path = Path::new(&to_path);
    let from_path = format!("./assets/{}.png", vanilla_chosen);
    let vanilla_path = Path::new(&from_path);
    match fs::copy(vanilla_path, final_path) {
        Err(er) => panic!("{}", er),
        Ok(_) => println!("{}", "Successfully installed Vanilla 💚".green()),
    };
    write_config(color, style);
}
