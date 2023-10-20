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
        // on récupère la saisie du joueur
        io::stdin()                                // on lit l'entrée standard écrit par le joueur
        .read_line(input)      // on stocke la chaine mutable dans input
        .expect("Failed to read input.");          // on affiche un message si il y a une erreur
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