/*
 *      Nom du fichier : input.rs
 *
 *      Créé le : 19 octobre 2023
 *      Auteur : Théo BENARD & Joshua MONTREUIL
 *      Projet : Démineur en Rust 
 *      Cours : Programmation Sure et Efficace
 */

/*
 *  Import des bibliothèques externes
 */
use std::io;
use std::io::BufReader;
use std::fs::File;
use crate::game::Config; 

/*
 *  Fonction pour lire la saisie du joueur
 */
pub fn read_user_input(input: &mut String) {
        loop {
                // on récupère la saisie du joueur
                let result = io::stdin().read_line(input);

                match result {
                        Ok(_) => {
                                // on sort de la boucle, la saisie a été lue avec succès.
                                break;
                        }
                        Err(error) => {
                                // on affiche un message d'erreur, une erreur s'est produite.
                                eprintln!("Failed to read input: {}", error);
                                continue;
                        }
                }
        }
}

/*
 *  IA : Fonction pour lire le fichier de configuration et gérer les erreurs
 */
pub fn read_config_from_file(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
        // on ouvre le fichier json.
        let file = File::open(file_path)?;
        // on créé un lecteur (BufReader) pour lire le fichier.
        let reader = BufReader::new(file);
        // on désérialise le contenu JSON dans la structure Config.
        let config: Config = serde_json::from_reader(reader)?;
        // on retourne la configuration lue depuis le fichier.
        Ok(config)
}