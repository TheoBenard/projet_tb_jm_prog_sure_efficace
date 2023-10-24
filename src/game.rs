/*
 *      Nom du fichier : game.rs
 *
 *      Créé le : 16 octobre 2023
 *      Auteur : Théo BENARD & Joshua MONTREUIL
 *      Projet : Démineur en Rust 
 *      Cours : Programmation Sure et Efficace
 */

/*
 *  Import des bibliothèques externes
 */
extern crate rand;          // génération de nombres aléatoires.
extern crate serde;         // sérialisation et la désérialisation de données.
extern crate serde_json;    // travailler avec le format JSON.

use std::collections::HashSet;
use rand::Rng;
use std::time::Instant;

use serde_derive::{Deserialize, Serialize};

mod display;
mod read;

/*
 *  Définition des Structures et des Énumérations
 */
#[derive(Clone, Copy)]
pub struct BoardInfo {
    pub(crate) board_size : usize,
    pub(crate) num_mines : usize,
}

pub struct Minesweeper {
    board: Vec<Vec<Cell>>,
    mines: HashSet<(usize, usize)>,
    pub(crate) revealed: HashSet<(usize, usize)>,
    pub(crate) game_over: bool,
    pub(crate) num_mark: usize,
    pub(crate) player_message: Option<String>,
    first_play : bool,
    configuration : BoardInfo,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cell {
    Mine,
    Number(u8),
    Empty,
    Undiscovered,
    Mark,
}
#[derive(Serialize, Deserialize)]
struct GameMode {
    name: String,
    board_size: u32,
    num_mines: u32,
    index: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    game_modes: Vec<GameMode>,
}

/*
 *  Définition du trait GameActions
 */
trait GameActions {
    fn reveal(&mut self, row: usize, col: usize);
    fn mark_mine(&mut self, row: usize, col: usize);
    fn initialize_game_on_first_play(&mut self, row: usize, col: usize);
    fn count_mines_around(&self, row: usize, col: usize) -> u8;
    fn check_win(&mut self) -> bool;
}

/*
 *  Implémentation des actions de jeu pour la structure Minesweeper
 */
impl GameActions for Minesweeper {
    /*
     *  Fonction pour révéler les cellules
     */
    fn reveal(&mut self, row: usize, col: usize) {
        // on vérifie si les coordonnées 'row' et 'col' sont hors de la grille de jeu.
        if row >= self.configuration.board_size || col >= self.configuration.board_size {
            return; // si c'est le cas, on sort de la fonction.
        }

        // on vérifie si la case a déjà été révélée.
        if self.revealed.contains(&(row, col)){
            return; // si elle a déjà été révélée, on sort de la fonction.
        }

        // on vérifie si la case contient une mine.
        if self.mines.contains(&(row, col)) {
            self.game_over = true;
            return; // si la case contient une mine, le jeu est terminé.        
        }

        // on compte le nombre de mines autour de la case.
        let mines_around = self.count_mines_around(row, col);
        self.revealed.insert((row, col));

        // on met à jour le tableau du jeu en fonction du nombre de mines autour de la case.
        self.board[row][col] = match mines_around {
            0 => Cell::Empty,
            n => Cell::Number(n),
        };

        // IA : si la case ne contient aucune mine à proximité (mines_around == 0).
        if mines_around == 0 {
            for r in row.saturating_sub(1)..=row + 1 {
                for c in col.saturating_sub(1)..=col + 1 {
                    // on révèle les cases adjacentes.
                    self.reveal(r, c);
                }
            }
        }
    }
    
    /*
     *  Fonction pour marquer une mine sur le plateau
     */
    fn mark_mine(&mut self, row: usize, col: usize) {
        // on vérifie si les coordonnées 'row' et 'col' sont en dehors des limites du tableau.
        if row >= self.configuration.board_size || col >= self.configuration.board_size {
            self.player_message = Some("The row and/or column are not within the game boundaries...".to_string());
            return; // si c'est le cas, on sort de la fonction en affichant un message au joueur.
        }
        
        // on vérifie si le joueur a déjà effectué sa première action de jeu.
        if !self.first_play{
            self.player_message = Some("You must reveal a square...".to_string());
            return; // si ce n'est pas le cas, on sort de la fonction en affichant un message au joueur.
        }

        // si la case est déjà révélée et marquée comme une mine, on la désactive en la changeant en une case non marquée.
        if self.revealed.contains(&(row, col)) && self.board[row][col] == Cell::Mark {
            match self.board[row][col] {
                Cell::Mark => {
                    self.board[row][col] = Cell::Mine; // on change la cellule d'état en mine.
                    self.revealed.remove(&(row, col)); // on retire le drapeau du tableau.
                    self.num_mark -= 1;                // on décrémente le nombre de mine.
                },
                _ => {
                    // on ne fait rien si la cellule contient autre chose qu'un drapeau.
                },
            }
        // sinon, si le nombre maximal de marquages de mines n'est pas atteint et que la case est non découverte ou une mine, on la marque comme un drapeau.
        } else if self.num_mark < self.configuration.num_mines && self.board[row][col] == Cell::Undiscovered || self.board[row][col] == Cell::Mine {
            match self.board[row][col] {
                Cell::Undiscovered | Cell::Mine => {
                    self.board[row][col] = Cell::Mark; // on change la cellule d'état en drapeau.
                    self.revealed.insert((row, col));  // on ajoute le drapeau au tableau.
                    self.num_mark += 1;                // on incrémente le nombre de mine.
                },
                _ => {
                    // on ne fait rien si la cellule contient autre chose que rien ou une mine.
                },
            }
        }
    }

    /*
     *   Fonction pour générer les mines au premier tour
     */
    fn initialize_game_on_first_play(&mut self, row: usize, col: usize) {
        // on vérifie si ce n'est pas le premier coup du joueur.
        if !self.first_play {

            // on onitialise un générateur de nombres aléatoires.
            let mut rng = rand::thread_rng();

            // on remplit la grille avec des mines jusqu'à atteindre le nombre de mines spécifié.
            while self.mines.len() < self.configuration.num_mines {

                // on génère des coordonnées aléatoires pour les mines.
                let random_row = rng.gen_range(0..self.configuration.board_size);
                let random_col = rng.gen_range(0..self.configuration.board_size);

                // IA : on déclare une variable pour vérifier que les coordonnées aléatoires de la mine ne sont pas adjacents à la première case jouée par le joueur.
                let is_adjacent = (random_row as isize - row as isize).abs() <= 1 && (random_col as isize - col as isize).abs() <= 1;

                // on vérifie que la mine générée n'est pas adjacente à la première case jouée par le joueur et qu'elle n'a pas déjà été révélée.
                if !is_adjacent && !self.revealed.contains(&(random_row, random_col)) {
                    // on ajoute la mine au plateau du démineur.
                    self.mines.insert((random_row, random_col));
                    // on déclare cette case comme étant une mine.
                    self.board[random_row][random_col] = Cell::Mine;
                }
            }    

            // on révèle les cases adjacentes à la première case jouée.
            for r in row.saturating_sub(1)..=row + 1 {
                for c in col.saturating_sub(1)..=col + 1 {
                    self.reveal(r as usize, c as usize);
                }
            }

            // on met à jour la valeur de la variable du premier coup du joueur.
            self.first_play = true;
        }
    }

    /*
     *  Fonction pour déterminer les numéros autours des mines
     */
    fn count_mines_around(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;

        // on parcourt les cases voisines (3x3) autour de la case spécifiée.
        for r in row.saturating_sub(1)..=row + 1 {
            for c in col.saturating_sub(1)..=col + 1 {
                // on vérifie si la case est à l'intérieur des limites du plateau et contient une mine.
                if r < self.configuration.board_size && c < self.configuration.board_size && self.mines.contains(&(r, c)) {
                    count += 1; // on incrémente le compteur de mines.
                }
            }
        }
        count // on retourne le nombre de mines trouvées autour de la case.
    }

    /*
     *   Fonction pour vérifier la victoire
     */
    fn check_win(&mut self) -> bool {
        // on calcule la condition de victoire en comparant le nombre de cases révélées et le nombre de cases marquées.
        self.revealed.len() - self.num_mark == (self.configuration.board_size * self.configuration.board_size) - self.configuration.num_mines
        // on retourne vrai si le joueur a gagné, sinon on retourne faux.
    }
}

/*
 *  Implémentation du constructeur de la structure Minesweeper 
 */
impl Minesweeper {
    pub fn new(game_config : BoardInfo) -> Self {
        let board = vec![vec![Cell::Undiscovered; game_config.board_size]; game_config.board_size];

        Minesweeper {
            board,
            mines: HashSet::new(),
            revealed: HashSet::new(),
            game_over: false,
            num_mark: 0,
            player_message: Some("".to_string()),
            first_play: false,
            configuration : game_config,
        }
    }
}

/*
 *  Fonction qui gère la saisie du joueur sur le plateau 
 */
pub fn play(game_config : BoardInfo) {
    // on déclare une nouvelle instance du jeu Minesweeper.
    let mut game_instance = Minesweeper::new(game_config);

    // on initialise une nouvelle chaîne modifiable.
    let mut input = String::new();

    // on initialise une nouvelle variable pour gérer le timer.
    let mut start_time: Option<Instant> = None;

    // on vérifie que le timer n'est pas encore initialisée.
    if let None = start_time {
        // on initialise `start_time` en lui attribuant la valeur actuelle de l'instant.
        start_time = Some(Instant::now());
    }
    
    // on continue tant que le joueur n'a ni perdu ni gagné.
    while !game_instance.game_over && !game_instance.check_win() {

        // on affiche le démineur.
        display::print_board(&mut game_instance);

        // on explique au joueur comment jouer.
        println!("Enter row and column (e.g., '2,0' , '12,19') or mark a mine (e.g., '3,3!' , '5,15!') :");

        // on efface la valeur d'input pour ne pas avoir de bug.
        input.clear();
        
        // on récupère la saisie du joueur.
        read::read_user_input(&mut input);

        // Traitement de la saisie utilisateur.
        let chars: Vec<char> = input.chars().collect();                             // On extrait chaque caractère de la saisie du joueur dans un vecteur de caractères.
        let before_last_chars = chars[chars.len() - 2];                       // On extrait le caractère avant le dernier de la saisie.
        let contains_commas = input.contains(',');                            // On vérifie si la saisie contient une virgule.
        let num_commas = input.chars().filter(|&c| c == ',').count();  // IA : on compte le nombre de virgules dans la saisie.
        let contains_exclamation = input.contains('!');                       // on vérifie si la saisie contient un point d'exclamation.
        let input = input.replace("!", "");                        // on supprime les points d'exclamation de la saisie.

        // on vérifie que le joueur a saisi une seule virgule.
        if contains_commas && num_commas == 1 {
        
            // IA : on divise la saisie en parties distinctes en utilisant la virgule comme séparateur et on les stocke dans un vecteur.
            let parts: Vec<&str> = input.trim().split(',').collect();

            // On regarde si le joueur veut découvrir une cellule ou poser un drapeau.
            if !contains_exclamation && parts.len() == 2 { // le joueur veut découvrir une cellule.

                // IA : on convertit les deux parties de la saisie (ligne et colonne) en valeurs numériques de type `usize`.
                if let (Some(row),
                        Some(col)) =
                            (parts[0].parse::<usize>().ok(),
                             parts[1].parse::<usize>().ok()) { 
                    // on vérifie que ligne et colonne sont compris dans le tableau BOARD_SIZE.
                    if row < game_instance.configuration.board_size && col < game_instance.configuration.board_size {

                        // si ligne et colonne sont compris dans le tableau, on vérifie si c'est le premier coup du joueur.
                        if !game_instance.first_play { 
                            // si c'est le premier coup, on initialise le jeu en disposant les mines de manière aléatoire.
                            game_instance.initialize_game_on_first_play(row, col);
                        } else {
                            // si ce n'est pas le premier coup, on vérifie que la case n'a pas été découverte.
                            if game_instance.board[row][col] == Cell::Undiscovered || 
                               game_instance.board[row][col] == Cell::Mine { 
                                // si oui, on révèle la case du jeu située à la position spécifiée par les valeurs de 'row' et 'col'.
                                game_instance.reveal(row as usize, col as usize);
                            } else {
                                // si non, on affiche un message d'erreur.
                                game_instance.player_message = Some("This square has already been revealed...".to_string());
                            }
                        }
                    } else {
                        // si la ligne et la colonne ne sont pas compris dans le tableau, on affiche un message d'erreur.
                        game_instance.player_message = Some("The row and/or column are not within the game boundaries...".to_string());
                    }
                } else {
                    // si le joueur a saisi une virgule mais pas de chiffre, on affiche un message d'erreur.
                    game_instance.player_message = Some("Please enter a digit or a number as per the instructions below...".to_string());
                }
            }

            if contains_exclamation && before_last_chars == '!' && parts.len() == 2 { // le joueur veut poser un drapeau.

                // IA : on convertit les deux parties de la saisie (ligne et colonne) en valeurs numériques de type `usize`.
                if let (Some(row),
                        Some(col)) =
                            (parts[0].parse::<usize>().ok(),
                             parts[1].parse::<usize>().ok()) { 

                    // on vérifie que ligne et colonne sont compris dans le tableau BOARD_SIZE.
                    if row < game_instance.configuration.board_size && col < game_instance.configuration.board_size { 
                        // on vérifie que la case n'a pas été découverte.
                        if game_instance.board[row][col] == Cell::Undiscovered || 
                           game_instance.board[row][col] == Cell::Mine || 
                           game_instance.board[row][col] == Cell::Mark {
                            // si oui, on ajoute un drapeau au tableau du démineur.
                            game_instance.mark_mine(row as usize, col as usize);
                        } else {
                            // si non, on affiche un message d'erreur.
                            game_instance.player_message = Some("This square has already been revealed...".to_string());
                        }
                    } else {
                        // si la ligne et la colonne ne sont pas compris dans le tableau, on affiche un message d'erreur.
                        game_instance.player_message = Some("The row and/or column are not within the game boundaries...".to_string());
                    }
                } else {
                    // si le joueur a saisi une virgule mais pas de chiffre, on affiche un message d'erreur.
                    game_instance.player_message = Some("Please enter a digit or a number as per the instructions below...".to_string());
                }
            }
        } else {
            // si le joeur n'a pas utilisé de virgule, on affiche un message d'erreur.
            game_instance.player_message = Some("You must adhere to the notation using a comma...".to_string());
        }

        if game_instance.game_over { // on vérifie la valeur de game_over (booléen) dans la structure de jeu.
            display::print_board(&mut game_instance); // on affiche le tableau de jeu.
            println!("Game Over ! You hit a mine.\n"); // on informe le joueur qu'il a perdu.
        }

        if game_instance.check_win() { // on vérifie si le joueur a découvert toutes les cases.
            display::print_board(&mut game_instance); // on affiche le tableau de jeu.
            // on mesure le temps écoulé depuis le point de départ.
            let elapsed_time = match start_time {
                Some(start) => start.elapsed(),
                None => {
                    // on gère le cas où `start_time` est `None` (non initialisé)
                    std::time::Duration::new(0, 0)
                }
            };
            println!("Congratulations ! You won ! \u{1F389}"); // on informe le joueur qu'il a gagné.
            // on affiche le temps que le joueur a passé dans un format lisible.
            println!("   Your time is {:02}:{:02}:{:03}\n", elapsed_time.as_secs() / 60, elapsed_time.as_secs() % 60, elapsed_time.subsec_millis());
        }
    }
}

fn play_again () -> u32 {
    // on initialise une nouvelle chaîne modifiable.
    let mut input = String::new();
    
    println!(" 1 : Play again with the same game mode.");
    println!(" 2 : Play again with a different game mode.");
    println!(" 3 : Exit the game.\n");

    loop {
        println!("Enter your choice (e.g., '1', '3') :");
        // on récupère la saisie du joueur.
        read::read_user_input(&mut input);

        // on analyse l'entrée de l'utilisateur.        
        match input.trim().parse() {
            // si le chiffre est compris entre 1 et 3 .
            Ok(nombre) if nombre == 1 || nombre == 2 || nombre == 3 => {
                // on renvoie le chiffre.
                return nombre; 
            }
            // si l'entrée n'est pas valide.
            _ => {
                // on efface l'entrée invalide.
                input.clear(); 
                continue;
            }
        };
    }
}

/*
 *  Fonction principale du jeu du Démineur 
 */
pub fn main_game() {
    // on déclare une variable pour suivre l'état du jeu.
    let mut is_game_on = true;

    // on initialise une variable pour stocker l'entrée du joueur depuis le menu.
    let mut user_menu_input: u32;
    // on initialise une variable pour stocker la décision du joueur de rejouer après une partie.
    let mut user_play_again: u32;

    // Gestion des erreurs lors de l'ouverture du fichier de configuration.
    let config = match read::read_config_from_file("src/config.json") {
        // on charge la configuration à partir du fichier si aucune erreur n'est rencontrée.
        Ok(conf) => conf, 
        Err(err) => {
            // on affiche une erreur en cas de problème de lecture.
            eprintln!("Incorrect configuration file reading error : {}", err);
            // on quitte la fonction en cas d'erreur.
            return;
        }
    };

    // IA : on trouve l'index maximal parmi les modes de jeu définis dans la configuration.
    let max_index = match config.game_modes.iter().map(|mode| mode.index).max() {
        Some(max_index) => max_index as u32,
        None => 0,
    };
    
    while is_game_on {
        // on affiche le menu et recueille l'entrée du joueur.
        user_menu_input = display::print_menu();
        
        if user_menu_input <= max_index {
            if user_menu_input == 1 {
                // on affiche les règles du jeu.
                display::print_game_rule();
            } else {
                // on recherche le mode de jeu sélectionné dans la configuration.
                let selected_mode = config.game_modes.iter().find(|&mode| mode.index == user_menu_input);

                match selected_mode {
                    Some(mode) => {
                        // on crée une structure BoardInfo pour stocker les détails du mode de jeu.
                        let board_info = BoardInfo {
                            board_size: mode.board_size as usize,
                            num_mines: mode.num_mines as usize,
                        };
                        // on entre dans une boucle pour jouer et permettre au joueur de rejouer.
                        loop {
                            // on commence une partie avec les informations du mode actuel.
                            play(board_info);

                            // on demande au joueur s'il souhaite rejouer.
                            user_play_again = play_again();
                            
                            if user_play_again == 2 {
                                // on sort de la boucle de jeu si le joueur ne souhaite pas rejouer.
                                break;
                            } else if user_play_again == 3 {
                                // on quitte le jeu si le joueur le choisit.
                                is_game_on = false;
                                break;
                            }
                        }    
                    }
                    None => {
                        // on affiche un message d'erreur si le mode de jeu sélectionné n'est pas trouvé dans la configuration.
                        eprintln!("Incorrect game mode.");
                    }
                }
            }
        } else if user_menu_input == 9 {
            // on quitte le jeu si le joueur sélectionne l'option 9.
            is_game_on = false;
        } else {
            // on affiche un message d'erreur en cas de choix invalide.
            eprintln!("Invalid choice. Please try again.");
        }
    }
}