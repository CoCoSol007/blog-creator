const express = require('express');
const cors = require('cors');
const fs = require('fs');

const app = express();

app.use(cors()); // Utilisez CORS pour gérer les en-têtes CORS

function sendJsonResponse(res, filePath) {
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
}

app.get('/data', (req, res) => {
    res.setHeader('Content-Type', 'application/json');
    const filePath = 'website/data/data.json';
    sendJsonResponse(res, filePath);
});

app.get('/articles', (req, res) => {
    res.setHeader('Content-Type', 'application/json');
    const filePath = 'website/data/articles.json';
    sendJsonResponse(res, filePath);
});

const port = 3000;

app.listen(port, () => {
    console.log(`Serveur en cours d'exécution sur le port ${port}`);
});
