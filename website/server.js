const express = require('express');
const cors = require('cors');
const fs = require('fs');

const app = express();

app.use(cors()); // Utilisez CORS pour gérer les en-têtes CORS

app.get('/', (req, res) => {
    res.setHeader('Content-Type', 'application/json');
    const filePath = 'data.json';

    fs.access(filePath, fs.constants.F_OK, (err) => {
        if (err) {
            const responseJson = {
                error: 'Le fichier demandé n\'existe pas.',
            };
            res.json(responseJson);
        } else {
            fs.readFile(filePath, 'utf8', (err, data) => {
                if (err) {
                    const responseJson = {
                        error: 'Une erreur s\'est produite lors de la lecture du fichier.',
                    };
                    res.json(responseJson);
                } else {
                    const jsonData = JSON.parse(data);
                    res.json(jsonData);
                }
            });
        }
    });
});

const port = 3000;

app.listen(port, () => {
    console.log(`Serveur en cours d'exécution sur le port ${port}`);
});
