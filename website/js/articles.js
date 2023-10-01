
function createH1WithButton(title, text, path) {
    const btn = document.createElement("button");

    const h1 = document.createElement("h3");
    h1.textContent = title;

    btn.appendChild(h1); // Ajoute le titre à l'intérieur du bouton
    btn.appendChild(document.createTextNode(text)); // Ajoute le texte du bouton

    // Définit l'attribut onclick avec une fonction de redirection
    btn.onclick = function () {
        window.location.href = path;
    };
    console.log(path);


    const targetDiv = document.getElementById("main");
    targetDiv.appendChild(btn); // Ajoute le <h1> à la fin du corps de la page
}


// Fonction pour lire le fichier CSV
function readJSONAndCreateH1() {
    fetch('/articles')
        .then(response => response.json())
        .then(data => {
            data.forEach(function (item) {
                const title = item.title;
                const text = item.description;
                const path = "/articles/" + item.title + ".html";
                createH1WithButton(title, text, path);
            });
        })
        .catch(error => console.error('Erreur de chargement du JSON : ARTICLES :', error));
}

// Appeler la fonction pour lire le CSV et créer les boutons
readJSONAndCreateH1();
