use colored::Colorize;
use std::{
    env,
    fs::{self, File},
    io::{self, Write},
    process::{Command, Stdio},
};

mod theme_data;

use theme_data::*;

fn check_if_in_termux() -> bool {
    env::var("TERMUX_VERSION").is_ok()
}

fn write_data_to_file(data: &str) -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = env::var("HOME")?;
    fs::create_dir_all(format!("{home_dir}/.termux"))?;
    let path = format!("{home_dir}/.termux/colors.properties");
    let mut file = File::create(path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

fn print_themes() {
    print_colored("1", "Base16 default dark");
    print_colored("2", "Base16 default light");
    print_colored("3", "Base16 greenscreen dark");
    print_colored("4", "Dracula");
    print_colored("5", "Gnometerm new");
    print_colored("6", "Gruvbox dark");
    print_colored("7", "Gruvbox light");
    print_colored("8", "Material");
    print_colored("9", "Solarized dark");
    print_colored("10", "Solarized light");
    print_colored("11", "Tokyo night dark");
    print_colored("12", "Tokyo night day");
    print_colored("13", "Tomorrow Night");
    print_colored("14", "Ubuntu");
    print_colored("15", "Zenburn");
    print_colored("r", "Reset");
    print_colored("q", "Quit");
}

fn print_colored(key: &str, text: &str) {
    println!("{:3}- {}", key.blue(), text.red());
}

fn match_user_input(user_input: &str) -> Option<&'static str> {
    match user_input {
        "1" => Some(BASE16_DEFAULT_DARK),
        "2" => Some(BASE16_DEFAULT_LIGHT),
        "3" => Some(BASE16_GREENSCREEN_DARK),
        "4" => Some(DRACULA),
        "5" => Some(GNOMETERM_NEW),
        "6" => Some(GRUVBOX_DARK),
        "7" => Some(GRUVBOX_LIGHT),
        "8" => Some(MATERIAL),
        "9" => Some(SOLARIZED_DARK),
        "10" => Some(SOLARIZED_LIGHT),
        "11" => Some(TOKYONIGHT_DARK),
        "12" => Some(TOKYONIGHT_DAY),
        "13" => Some(TOMORROW_NIGHT),
        "14" => Some(UBUNTU),
        "15" => Some(ZENBURN),
        _ => None,
    }
}

fn reload_termux() -> Result<(), std::io::Error> {
    Command::new("am")
        .args([
            "broadcast",
            "--user",
            "0",
            "-a",
            "com.termux.app.reload_style",
            "com.termux",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    Ok(())
}

fn delete_colors_file() -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = env::var("HOME")?;
    let path = format!("{home_dir}/.termux/colors.properties");
    fs::remove_file(path)?;
    reload_termux()?;
    Ok(())
}

fn apply_theme(theme_data: &str) -> Result<(), Box<dyn std::error::Error>> {
    write_data_to_file(theme_data)?;
    reload_termux()?;
    Ok(())
}

fn main() {
    if !check_if_in_termux() {
        eprintln!("Termux environment not found");
        std::process::exit(1);
    }
    print_themes();

    let mut user_input = String::new();
    print!("Enter theme: ");
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Error reading from STDIN");
    let user_input = user_input.trim();

    if user_input == "q" {
        std::process::exit(0);
    } else if user_input == "r" {
        if let Err(e) = delete_colors_file() {
            eprintln!("Error: {e}");
        } else {
            println!("Theme reset")
        }
    } else if let Some(theme_data) = match_user_input(user_input) {
        match apply_theme(theme_data) {
            Ok(_) => println!("Theme updated"),
            Err(e) => eprintln!("Error: {e}"),
        }
    } else {
        eprintln!("Invalid input");
    }
}
