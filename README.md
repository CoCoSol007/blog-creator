# Article Management Project
This Rust project has been developed to manage and display articles stored in a JSON file. It includes a command-line interface for performing various operations such as creating, deleting, listing, and displaying articles. Additionally, there is an interactive HTML page that uses JavaScript to display the stored articles.

# Features
The project includes the following features:

Creation of articles with a title, description, and automatic date.
Deletion of articles based on their index.
Listing of all existing articles.
Detailed display of a specific article.
Simple command-line user interface.
# Usage
- first, you need to run a server made in Java-Scrpite :
```bash
// To compile and run the Java Scripte program (server)
node website/server.js
```


- To run the command-line program, use the following commands:
```bash
// To compile and run the Rust program
cargo build
cargo run
```

- Follow the on-screen instructions to manage your articles.

- To display the articles in a web page, open the index.html file in a modern web browser. Ensure that the JSON file containing article data (data.json) is present in the "website" directory.

 # Dependencies
Rust (programming language)
Rust libraries: serde, serde_json, chrono
HTML and JavaScript (for displaying articles)
# Project Structure
src/main.rs: Main source code for the command-line program.
website/index.html: HTML page for displaying articles.
website/script.js: JavaScript for loading and displaying articles from the JSON file.
website/style.css: Stylesheet for the web page layout.
website/data.json: JSON file containing article data.
