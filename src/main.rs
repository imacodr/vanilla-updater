mod verify_studio;

use colored::*;
use verify_studio::check_studio;

fn main() {
    let icecream_text_art = r#"
    .-"`'"-.
    /        \
    |        |
    /'---'--`\
   |          |
   \.--.---.-./
   (_.--._.-._)
     \=-=-=-/
      \=-=-/
       \=-/
        \/

Loading Vanilla Icecream...
    "#;
    let print_to_text = format!("{}", icecream_text_art);
    println!("{}", print_to_text.bold().white());
    check_studio();
}
