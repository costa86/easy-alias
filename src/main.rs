use home::home_dir;
use std::process::{self, Command};
use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
};

fn exit_program(message: &str) {
    println!("{}", message);
    process::exit(1);
}

fn get_alias_file_path() -> String {
    format!("{}/aliases", home_dir().unwrap().display(),)
}

fn get_file_contents() -> File {
    let aliases_file_path = get_alias_file_path();

    if File::open(&aliases_file_path).is_err() {
        exit_program(
            format!(
                "File not found: {}.\nMake sure this file exists.",
                aliases_file_path
            )
            .as_str(),
        )
    }

    let aliases_file_content = File::open(&aliases_file_path).unwrap();
    let buffer = BufReader::new(&aliases_file_content);

    if buffer.lines().count() == 0 {
        exit_program(format!("File {} is empty", aliases_file_path).as_str());
    }

    let aliases_file_path = get_alias_file_path();
    let aliases_file_content = File::open(&aliases_file_path).unwrap();
    aliases_file_content
}

fn get_commands() -> Vec<String> {
    let file_contents = get_file_contents();
    let buffer = BufReader::new(&file_contents);
    let mut index = String::new();
    let error_msg = "Invalid command";
    let mut command_list: Vec<String> = buffer.lines().map(|x| x.unwrap()).collect();
    command_list.sort();

    command_list.iter().for_each(|x| {
        println!(
            "{} - {}",
            command_list.iter().position(|y| y == x).unwrap(),
            x,
        )
    });

    stdin().read_line(&mut index).unwrap();

    let index = index.split("\n").next().unwrap();

    if index.len() == 0 {
        exit_program(&error_msg);
    }

    let index = index.parse();
    if index.is_err() {
        exit_program(&error_msg);
    }
    let index: usize = index.unwrap();

    let commands = command_list.get(index);

    if commands.is_none() {
        exit_program(&error_msg);
    }

    let commands: Vec<String> = commands
        .unwrap()
        .split_whitespace()
        .map(str::to_string)
        .collect();
    commands
}

fn run_command(commands: Vec<String>) {
    let command_list = commands.split_first().unwrap();
    let command = command_list.0;
    let arguments = command_list.1;
    let error_msg = "command failed to start";

    match commands.len() {
        1 => Command::new(&command).status().expect(&error_msg),
        _ => Command::new(&command)
            .args(arguments)
            .status()
            .expect(&error_msg),
    };
}

fn display_app_intro() {
    let title = format!(
        "\n{} - {} \nAuthors: {}\nVersion: {}\nLicense: {}\nCrafted with ❤️ using Rust language\nSet the commands in {}\n",
        env!("CARGO_PKG_NAME").to_uppercase(),
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_LICENSE"),
        get_alias_file_path()
    );
    println!("{title}");
}

fn main() {
    display_app_intro();
    println!("Run a command based on its index:\n");
    run_command(get_commands())
}
