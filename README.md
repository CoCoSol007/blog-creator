# Article Management Project
This project has been developed to manage and display articles. It includes a command-line interface for performing various operations such as creating, deleting, listing, and displaying articles. Additionally, there is an interactive HTML page that uses JavaScript to display the stored articles.

In the rust programmme there is an [IDE](https://github.com/Kofituo/pound/blob/search_arrows/src/main.rs). commands:
- ctr+c: quit
- ctr+s: save

# HELP
there are 8 commands in the command-line rust programme:

- quit: Quit the program.
- list: List all articles.
- clear: Clear the console screen.
- show <index>: Show the details of the article at the specified index.
- del <index>: Delete the article at the specified index
- wirte <index>: wirte or edite an article at the specified index.
- create: Create a new article.
- help: Display this help message.

# Features
The project includes the following features:

Creation of articles with a title, description, and automatic date.
Deletion of articles based on their index.
Listing of all existing articles.
Detailed display of a specific article.
IDE to write a lot of articles in terminal !
Simple command-line user interface.
# Usage
- first, you need to run a server made in Java-Scrpite :
```bash
// To compile and run the Java Scripte program (server)
node website/server.js
```
- after that you can go on any browser and tap this url:
```
http://localhost:3000/
```


- To run the command-line program, use the following commands:
```bash
// To compile and run the Rust program
cargo build
cargo run
```

- Follow the on-screen (tap : "help") instructions to manage your articles.


 # Dependencies
Rust (programming language),
Rust libraries: serde, serde_json, chrono...
HTML, CSS and JavaScript (for displaying articles),
node for javaScripte.
# Project Structure
src/: Main source code for the command-line program.
website/: Website part with HTML page, css style, JavaScripte server and json data 
