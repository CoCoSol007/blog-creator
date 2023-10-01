
function get_name() {
    return fetch('/data')
        .then(response => {
            if (!response.ok) {
                throw new Error('Réponse du serveur non valide');
            }
            return response.json();
        })
        .then(data => {
            const name = data.name;
            return name;
        })
        .catch(error => {
            console.error('Erreur de chargement du JSON :', error);
            throw error; // Vous pouvez choisir de gérer l'erreur ici ou la propager plus haut
        });
}


get_name()
    .then(name => {
        const h1 = document.createElement("h1");
        h1.textContent = name;
        const targetDiv = document.getElementById("title");
        targetDiv.appendChild(h1); // Ajoute le <h1> à la fin du corps de la page
    })
    .catch(error => {
        console.error('Erreur lors de la récupération du nom :', error);
    });
