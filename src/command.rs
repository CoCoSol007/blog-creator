use chrono::Local;
use clear_screen::clear;
use serde_derive::{Deserialize, Serialize};
use std::fs::{remove_file, File};
use std::io::{self, Read, Write};
use crate::ide::run;




#[derive(Debug, Deserialize, Serialize)]
struct Article {
    title: String,
    description: String,
    path: String,
    date: String,
}

pub enum Command {
    Quit,
    List,
    Clear,
    Wirte(u8),
    Show(u8),
    Del(u8),
    Create,
    Help,
    Edit,
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
            Command::Wirte(index) => wirte(index),
            Command::Edit => todo!()
        }
    }
}

fn wirte(index:u8){
    let articles = open_json();
    let id = index- 1;
    if (id as usize )< articles.len() {
        let article_title = &articles[id as usize].title;
        let path = format!("website/articles/{article_title}");
        run(path);
        clear_console()
    } else {
        println!("Index {} doesn't exist, they have just:", id + 1 );
        list();
    }
}

fn help() {
    println!("Available commands:");
    println!("quit: Quit the program.");
    println!("list: List all articles.");
    println!("clear: Clear the console screen.");
    println!("show <index>: Show the details of the article at the specified index.");
    println!("del <index>: Delete the article at the specified index.");
    println!("wirte <index>: wirte or edite an article at the specified index.");
    println!("create: Create a new article.");
    println!("edit: edit the website (color...)");
    println!("help: Display this help message.");
}

fn list() {
    let articles = open_json();
    if articles.is_empty() {
        println!("You don't have any articles!");
    } else {
        for (index, article) in articles.iter().enumerate() {
            println!("{} : {}", index + 1, article.title);
        }
    }
}

fn show_index(index: u8) {
    let articles = open_json();
    if let Some(article) = articles.get((index -1) as usize) {
        println!("Title: {}", article.title);
        println!("Description: {}", article.description);
        println!("Path: {}", article.path);
        println!("Date: {}", article.date);
    } else {
        println!("This index does not exist, but you have:");
        list();
    }
}

fn open_json() -> Vec<Article> {
    match File::open("website/data/articles.json") {
        Ok(mut file) => {
            let mut json_data = String::new();
            if file.read_to_string(&mut json_data).is_err() {
                eprintln!("Unable to read the JSON file.");
                return Vec::new();
            }
            if json_data.trim().is_empty() {
                return Vec::new();
            }

            match serde_json::from_str(&json_data) {
                Ok(articles) => articles,
                Err(e) => {
                    eprintln!("JSON deserialization error: {}", e);
                    Vec::new()
                }
            }
        }
        Err(e) => {
            eprintln!("Unable to open the JSON file : {}", e);
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
    let id = id - 1;
    if (id as usize )< articles.len() {
        let article_path = &articles[id as usize].path;
        let article_title= &articles[id as usize].title;

        if let Err(e) = remove_file(format!("website/{}", article_path)) {
            eprintln!("Error deleting HTML file: {}. path : website/{}", e, article_path);
        } else {
            println!("HTML file remove");
        }
        if let Err(e) = remove_file(format!("website//articles/{}", article_title)) {
            eprintln!("Error deleting Text file: {}. path : website/articles/{}", e, article_title);
        } else {
            println!("Text file remove");
        }


        articles.remove(id as usize);

        sync_article(articles)

    } else {
        println!("Index {} doesn't exist, they have just:", id + 1 );
        list();
    }
}

fn create_article() {

    println!("Enter the title:");
    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read input");

    println!("Enter the description:");
    let mut description = String::new();
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read input");


    let date = Local::now();

    let new_article = Article {
        title: title.trim().to_string(),
        description: description.trim().to_string(),
        path: format!("articles/{}.html", title.trim()),
        date: date.to_string(),
    };

    let mut articles = open_json();
    articles.push(new_article);

    sync_article(articles);
    create_html_file(title, description);

    
}

fn sync_article(articles: Vec<Article>)
{
    if let Ok(file) = File::create("website/data/articles.json") {
        if serde_json::to_writer_pretty(file, &articles).is_err() {
            eprintln!("Unable to write to the JSON file.");
        }else{
            println!("data updated successfully!");
        }
    } else {
        eprintln!("Unable to create the JSON file.");
    }
}

fn create_html_file(title: String,description:String) {
    let title_without_newlines = title.replace("\n", "");
    let html_content = format!(
        r#"<!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>{}</title>
            <link rel="stylesheet" href="/style.css">
        </head>
        <body>
            <div id="main">
                <h1>{}</h1>
                <h2>{}</h2>
    
                <button onclick="window.location.href='/'">HOME</button><br>
                <div id="article">
                    <script>
                        function get_name() {{
                            return fetch('/articles/{}')
                                .then(response => {{
                                    if (!response.ok) {{
                                        throw new Error('Réponse du serveur non valide');
                                    }}
                                    return response.text();
                                }})
                                .catch(error => {{
                                    console.error('Erreur de chargement du texte :', error);
                                    throw error;
                                }});
                        }}
                        
                        get_name()
                            .then(text => {{
                                const pre = document.createElement("pre"); 
                                pre.textContent = text;
                                const targetDiv = document.getElementById("article");
                                targetDiv.appendChild(pre);
                            }})
                            .catch(error => {{
                                console.error('Erreur lors de la récupération du texte :', error);
                            }});
                    </script>
                </div>
            </div>
        </body>
        </html>"#,
        title, title, description, title_without_newlines
    );
    
    if let Ok(mut html_file) =
                File::create(format!("website/articles/{}.html", title.trim()))
            {
                html_file
                    .write_all(html_content.as_bytes())
                    .expect("Failed to write HTML file");
                println!("New HTML page created successfully!");
            } else {
                eprintln!("Unable to create HTML file.");
            }
    if let Ok(_txt_file) =
            File::create(format!("website/articles/{}", title.trim()))
        {
            
            println!("New texte file page created successfully!");
        } else {
            eprintln!("Unable to create HTML file.");
        }
}