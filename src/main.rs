pub mod command;
pub mod ide;
use crate::command::{clear_console, Command};

fn main() {
    clear_console();
    loop {
        if let Some(command) = get_command() {
            command.run()
        } else {
            println!("Command don't know !")
        }
        println!();
    }
}

fn get_command() -> Option<Command> {
    let command = get_input();
    if let Some(index) = command.find("show ") {
        let resultat = &command[(index + "show ".len())..].trim();
        if let Ok(valeur) = resultat.parse::<u8>() {
            return Some(Command::Show(valeur));
        } else {
            println!("Wrong argument ! (you must give an index)");
        }
    }
    if let Some(index) = command.find("del ") {
        let resultat = &command[(index + "del ".len())..].trim();
        if let Ok(valeur) = resultat.parse::<u8>() {
            return Some(Command::Del(valeur));
        } else {
            println!("Wrong argument ! (you must give an index)");
        }
    }

    if let Some(index) = command.find("write ") {
        let resultat = &command[(index + "write ".len())..].trim();
        if let Ok(valeur) = resultat.parse::<u8>() {
            return Some(Command::Wirte(valeur));
        } else {
            println!("Wrong argument ! (you must give an index)");
        }
    }
    match command.trim() {
        "list" => Some(Command::List),
        "quit" => Some(Command::Quit),
        "clear" => Some(Command::Clear),
        "create" => Some(Command::Create),
        "help" => Some(Command::Help),
        "edit" => Some(Command::Edit),
        _ => None,
    }
}

fn get_input() -> String {
    let mut command = String::new();
    let _ = std::io::stdin().read_line(&mut command).unwrap();
    command
}
