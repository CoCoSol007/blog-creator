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

const path = require('path'); // Importez le module path

app.get('/style.css', (req, res) => {
    const filePath = path.join(__dirname, 'articles', 'style.css'); // Construire un chemin absolu
    res.setHeader('Content-Type', 'text/css'); // Définir le type de contenu comme CSS
    res.sendFile(filePath);
});


app.get('/articles.js', (req, res) => {
    const filePath = path.join(__dirname, 'js', 'articles.js'); // Construire un chemin absolu
    res.setHeader('Content-Type', 'application/javascript'); // Définir le type de contenu comme JavaScript
    res.sendFile(filePath);
});

app.get('/name.js', (req, res) => {
    const filePath = path.join(__dirname, 'js', 'name.js'); // Construire un chemin absolu
    res.setHeader('Content-Type', 'application/javascript'); // Définir le type de contenu comme JavaScript
    res.sendFile(filePath);
});



app.get('/', (req, res) => {
  const nomFichier = req.params.nomFichier;
  const cheminFichier = path.join(__dirname, "main.html");

  fs.readFile(cheminFichier, 'utf8', (err, data) => {
    if (err) {
      res.status(404).send('Fichier non trouvé');
    } else {
      res.send(data);
    }
  });
});

app.get('/articles/:nomFichier', (req, res) => {
    const nomFichier = req.params.nomFichier;
    const cheminFichier = path.join(__dirname, "articles",nomFichier);
  
    fs.readFile(cheminFichier, 'utf8', (err, data) => {
      if (err) {
        res.status(404).send('Fichier non trouvé');
      } else {
        res.send(data);
      }
    });
  });

  app.get('/index/:nomFichier', (req, res) => {
    const nomFichier = req.params.nomFichier;
    const cheminFichier = path.join(__dirname,"index", nomFichier);
  
    fs.readFile(cheminFichier, 'utf8', (err, data) => {
      if (err) {
        res.status(404).send('Fichier non trouvé');
      } else {
        res.send(data);
      }
    });
  });

const port = 3000;

app.listen(port, () => {
    console.log(`Serveur en cours d'exécution sur le port ${port}`);
});
