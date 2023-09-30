use chrono::Local;
use clear_screen::clear;
use serde_derive::{Deserialize, Serialize};
use std::fs::{remove_file, File};
use std::io::Read;
use std::io::{self, Write};

pub enum Command {
    Quit,
    List,
    Clear,
    Show(u8),
    Del(u8),
    Create,
    Help,
}

#[derive(Debug, Deserialize, Serialize)]
struct Article {
    title: String,
    description: String,
    path: String,
    date: String,
}

impl Command {
    pub fn run(self) {
        match self {
            Command::Quit => exit(),
            Command::List => list(),
            Command::Clear => clear_console(),
            Command::Show(index) => show_index(index),
            Command::Del(index) => sup_index_by_data(index),
            Command::Create => create_article(),
            Command::Help => help(),
        }
    }
}

fn help() {
    println!("Available commands:");
    println!("quit: Quit the program.");
    println!("list: List all articles.");
    println!("clear: Clear the console screen.");
    println!("show <index>: Show the details of the article at the specified index.");
    println!("del <index>: Delete the article at the specified index.");
    println!("create: Create a new article.");
    println!("help: Display this help message.");
}
fn list() {
    let articles = open_json();
    if articles.is_empty() {
        println!("You don't have any articles!")
    } else {
        let mut index: u8 = 1;
        for article in articles {
            println!("{} : {}", index, article.title);
            index += 1;
        }
    }
}

fn show_index(index: u8) {
    let articles = open_json();
    let mut found = false;
    let mut id: u8 = 1;
    for article in articles {
        if index == id {
            println!("Title: {}", article.title);
            println!("Description: {}", article.description);
            println!("Path: {}", article.path);
            println!("Date: {}", article.date);
            found = true;
        }
        id += 1;
    }
    if !found {
        println!("This index does not exist, but you have:");
        list();
    }
}

fn open_json() -> Vec<Article> {
    // Ouvrir le fichier JSON
    match File::open("../website/data/articles.json") {
        Ok(mut file) => {
            let mut json_data = String::new();
            if file.read_to_string(&mut json_data).is_err() {
                eprintln!("Unable to read the JSON file.");
                return Vec::new();
            }
            if json_data.trim().is_empty() {
                return Vec::new();
            }

            // Désérialiser la chaîne JSON en une structure Rust
            match serde_json::from_str(&json_data) {
                Ok(articles) => articles,
                Err(e) => {
                    eprintln!("JSON deserialization error: {}", e);
                    Vec::new()
                }
            }
        }
        Err(e) => {
            eprintln!("Unable to open the JSON file : {}",e);
            Vec::new()
        }
    }
}

fn exit() {
    clear();
    std::process::exit(0);
}

pub fn clear_console() {
    clear();
    println!("Made by CoCo_Sol");
}

fn sup_index_by_data(id: u8) {
    let mut articles = open_json();
    let index = id - 1;
    if (index as usize) < articles.len() {
        // Récupérez le chemin du fichier HTML associé à l'article
        let article_path = &articles[index as usize].path;

        // Supprimez le fichier HTML s'il existe
        if let Err(e) = remove_file(format!("../website/articles/{}", article_path)) {
            eprintln!("Error deleting HTML file: {}", e);
        }

        articles.remove(index as usize);
        if let Ok(file) = File::create("../website/data/articles.json") {
            // Sérialisez le vecteur d'articles en format JSON et écrivez-le dans le fichier
            if serde_json::to_writer_pretty(file, &articles).is_err() {
                eprintln!("Unable to write to the JSON file.");
            } else {
                println!("Article deleted successfully!");
            }
        } else {
            eprintln!("Unable to create the JSON file.");
        }
    } else {
        println!("Index {} doesn't exist, they have just:", id);
        list();
    }
}

fn create_article() {
    println!("Create a new article:");

    // Demandez à l'utilisateur de saisir le titre de l'article
    println!("Enter the title:");
    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read input");

    // Demandez à l'utilisateur de saisir la description de l'article
    println!("Enter the description:");
    let mut description = String::new();
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read input");

    // Demandez à l'utilisateur de saisir la date de l'article
    println!("Enter the date:");

    let date = Local::now();

    let html_content = format!(
        r#"<!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <title>{}</title>
                    <link rel="stylesheet" href="style.css">
                </head>
                <body>
                    <div id="main">
                        <h1>{}</h1>
                        <h2>{}</h2>

                        <a href="../main.html">Home</a>
                    </div>
                </body>
                </html>"#,
        title, title, description
    );

    // Créez une nouvelle instance d'Article avec les données de l'utilisateur
    let new_article = Article {
        title: title.trim().to_string(),
        description: description.trim().to_string(),
        path: format!("articles/{}.html", title.trim()),
        date: date.to_string(),
    };

    // Ajoutez le nouvel article à la liste existante
    let mut articles = open_json();
    articles.push(new_article);

    // Enregistrez la liste mise à jour dans le fichier JSON
    if let Ok(file) = File::create("../website/data/articles.json") {
        if serde_json::to_writer_pretty(file, &articles).is_err() {
            eprintln!("Unable to write to the JSON file.");
        } else {
            // Créez un fichier HTML vide avec un nom basé sur le titre de l'article
            if let Ok(mut html_file) =
                File::create(format!("../website/articles/{}.html", title.trim()))
            {
                html_file
                    .write_all(html_content.as_bytes())
                    .expect("Failed to write HTML file");
                println!("New article created successfully!");
            } else {
                eprintln!("Unable to create HTML file.");
            }
        }
    } else {
        eprintln!("Unable to create the JSON file.");
    }
}
