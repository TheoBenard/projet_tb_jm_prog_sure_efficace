/*
 *      Nom du fichier : display.rs
 *
 *      Créé le : 16 octobre 2023
 *      Auteur : Théo BENARD & Joshua MONTREUIL
 *      Projet : Démineur en Rust 
 *      Cours : Programmation Sure et Efficace
 */

/*
 *  Import des bibliothèques externes
 */
use std::io;
use termion::{color, style};

use crate::game::{Cell, Minesweeper};
use super::read;

/*
 *  Fonction pour afficher un message d'erreur au joueur
 */
pub fn print_message_and_clear(minesweeper_info: &mut Minesweeper) {
    // on regarde si la variable "player_message" contient une valeur.
    if let Some(message) = &minesweeper_info.player_message {
        // si oui, on affiche la valeur du message.
        println!("\n{}\n", message);
        // on réinitialise la variable à une chaîne vide.
        minesweeper_info.player_message = Some("".to_string());
    }
}

/*
 *  Fonction pour effacer l'écran
 */
pub fn clean_screen() {
    // on vérifie si l'effacement de l'écran a rencontré une erreur.
    if let Err(err) = crossterm::execute!(
            // on sélectionne la sortie standard du terminal que l'on veut effacé.
            io::stdout(),
            // IA : on envoie une commande pour effacer tout le contenu de l'écran.
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
        ) {
        // on affiche un message d'erreur avec la description de l'erreur
        eprintln!("Error while clearing the screen : {}", err);
    }
}

/*
 *  Fonction pour l'affichage du Démineur
 */
pub fn print_board(minesweeper_info: &mut Minesweeper) {
    // on efface l'écran à chaque tour
    clean_screen();

    println!("   Minesweeper game \u{1F579}\u{FE0F}\n");

    // on affiche les numéros des colonnes
    let mut row_count = 0;
    let mut tens_count = 0;
    print!("   ");
    for _col in &minesweeper_info.board {
        if tens_count == 0 {
            print!("  ");
        } else {
            print!(" {}", tens_count);
        }
        if row_count == 9 {
            tens_count += 1;
            row_count = 0;
        } else {
            row_count += 1;
        }
    }
    row_count = 0;
    print!("\n   ");
    for _col in &minesweeper_info.board {
        print!(" {}", row_count);
        if row_count == 9 {
            row_count = 0;
        } else {
            row_count += 1;
        }
    }
    print!("\n");

    // on affiche le plateau du jeu
    row_count = 0;
    for row in &minesweeper_info.board {
        print!("{}  ", row_count);
        if row_count < 10 {
            print!(" ");
        }
        for col in 0..minesweeper_info.configuration.board_size {
            let cell = &row[col];
            if minesweeper_info.game_over || minesweeper_info.revealed.contains(&(row_count, col)) {
                match cell {
                    Cell::Undiscovered => print!(". "),
                    Cell::Number(num) => {
                        // on applique des couleurs aux chiffres
                        let colored_number = match num {
                            1 => format!("{}{}1{}", color::Fg(color::Green),       style::Bold, style::Reset),
                            2 => format!("{}{}2{}", color::Fg(color::Yellow),      style::Bold, style::Reset),
                            3 => format!("{}{}3{}", color::Fg(color::LightYellow), style::Bold, style::Reset),
                            4 => format!("{}{}4{}", color::Fg(color::LightRed),    style::Bold, style::Reset),
                            5 => format!("{}{}5{}", color::Fg(color::Red),         style::Bold, style::Reset),
                            6 => format!("{}{}6{}", color::Fg(color::Magenta),     style::Bold, style::Reset),
                            _ => num.to_string(), // chiffres non colorés
                        };
                        print!("{} ", colored_number);
                    }
                    Cell::Mine => {
                        if minesweeper_info.game_over {
                            print!("\x08\u{1F4A3} ");
                        } else {
                            print!(". "); // on cache les mines non révélées
                        }
                    }
                    Cell::Empty => print!("* "),
                    Cell::Mark => print!("\u{1F6A9}"),
                }
            } else {
                print!(". "); // on cache les mines non révélées
            }
        }
        if row_count == 2 {
            // on affiche le nombre de drapeau que le joueur peut encore poser
            print!("    Flag {}/{} \u{1F6A9}",minesweeper_info.num_mark, minesweeper_info.configuration.num_mines)
        }
        println!();
        // on incrémente le nombre de ligne
        row_count += 1;
    }
    // on affiche le message d'erreur s'il y en a un
    print_message_and_clear(minesweeper_info);
}

/*
 *  Fonction pour l'affichage du Menu
 */
pub fn print_menu() -> u32 {
    // on déclare une variable mutable pour gérer la saisie du joueur
    let mut input = String::new();

    // on déclare une variable mutable pour gérer les messages d'erreur
    let mut invalid_input_message = String::new();
    
    // on rentre dans une boucle tant que le joueur n'a pas choisi le bon mode
    loop { 
        clean_screen(); // on efface l'écran 
        println!("      \x08\u{1F4A3} Welcome to the Minesweeper game ! \u{1F6A9}\n");
        println!("Please press any key given below to continue :\n");
        println!(" 1 : Minesweeper game rule and How to play. \u{1F4DC}");
        println!(" 2 : Small 8x8 grid with 10 mines.");
        println!(" 3 : Medium 16x16 grid with 40 mines.");
        println!(" 4 : Large 32x32 grid with 160 mines.");
        println!(" 5 : Xtreme 16x16 grid with 246 mines. \u{2620}\u{FE0F}\n");
        println!(" Soon : configure your own grid...\n");
        println!(" 9 : Quit game. \u{1F6AA}\n");

        // on regarde si la variable contenant le message d'erreur est vide
        if !invalid_input_message.is_empty() {
            // si non, on affiche le message d'erreur au joueur
            println!("{}{}{}{}\n", color::Fg(color::LightRed), style::Bold, invalid_input_message,style::Reset );
            // on supprime la valeur du message d'erreur pour ne pas l'afficher 2 fois d'affiler
            invalid_input_message.clear();
        }

        println!("Enter your choice (e.g., '1', '4'):");

        // on récupère la saisie du joueur
        read::read_user_input(&mut input);

        // on analyse l'entrée de l'utilisateur        
        match input.trim().parse() {
            // si le chiffre est compris entre 1 et 5 
            Ok(nombre) if nombre >= 1 && nombre <= 5 => {
                // on renvoie le chiffre
                return nombre; 
            }
            // si le chiffre vaut 9 
            Ok(9) => {
                // on renvoie 9
                return 9;
            }
            // si l'entrée n'est pas valide
            _ => {
                // on définit un message d'erreur
                invalid_input_message = "The input is not valid. Please try again.".to_string();
                // on efface l'entrée invalide
                input.clear(); 
                continue;
            }
        };
    }
}

/*
 *  Fonction pour l'affichage des Règles du Jeu
 */
pub fn print_game_rule() -> u32 {
    // on déclare une variable mutable pour gérer la saisie du joueur
    let mut input = String::new();

    // on déclare une variable mutable pour gérer les messages d'erreur
    let mut invalid_input_message = String::new();

    // on rentre dans une boucle tant que le joueur n'a pas ce mode
    loop {
        clean_screen(); // on efface l'écran
        println!("      \x08\u{1F4A3} Welcome to the game rule of the Minesweeper game ! \u{1F6A9}\n\n");
        println!("  The goal of this game is to find every mines without clicking on them.");
        println!("  You will be on a grid with undiscovered tiles.\n");
        println!("  To begin, you need to enter the index of a row and then the index of a column (e.g., '2,0', '12,19').");
        println!("  This will be your first move. This move will uncover tiles and you will see some number appearing.");
        println!("  Those number indicates that there is a mine near the tile where the number is.");
        println!("  The tile where the mine is, is 'touching' the tiles where the number is.");
        println!("  If you want to uncover a tile where a mine is, then you lose.\n");
        println!("  To win, you will need to let uncovered the tile where you think the mines are. ");
        println!("  To ease the things for you, you can mark the mines with a flag where you think the mines are.");
        println!("  (Enter a row then col with '!' for a flag (e.g., '3,3!' , '5,15!')).\n");
        println!("  There is an example below :\n");
        println!("      . . .           * * * ");
        println!("      . {}{}1{} .           * {}{}1{} * ",color::Fg(color::LightGreen), style::Bold, style::Reset, color::Fg(color::LightGreen), style::Bold, style::Reset);
        println!("      . . .           * * \x08\u{1F4A3} \n");
        println!("  This '{}{}1{}' indicates that there is 1 mines in is peripherical tiles.", color::Fg(color::LightGreen), style::Bold, style::Reset);
        println!("  The mine is in the bellow right corner, so all of the other tiles are empty.\n\n");
        println!("  There is an example with a 3 below : \n");
        println!("      . . .           \x08\u{1F4A3} * * ");
        println!("      . {}{}3{} .           * {}{}3{} * ", color::Fg(color::LightYellow), style::Bold, style::Reset, color::Fg(color::LightYellow), style::Bold, style::Reset);
        println!("      . . .           * \x08\u{1F4A3} \x08\u{1F4A3} \n");
        println!("  This '{}{}3{}' indicates that there are 3 mines in is peripherical tiles.\n", color::Fg(color::LightYellow), style::Bold, style::Reset);
            
        // on regarde si la variable contenant le message d'erreur est vide
        if !invalid_input_message.is_empty() {
            // Si ce n'est pas le cas, on affiche le message d'erreur au joueur
            println!("{}{}{}{}\n", color::Fg(color::LightRed), style::Bold, invalid_input_message,style::Reset );
            // On supprime le message d'erreur pour ne pas l'afficher deux fois de suite
            invalid_input_message.clear();
        }

        println!("You are ready, press 1 to get out of this menu.");

        // on récupère la saisie du joueur
        read::read_user_input(&mut input);

        // on analyse l'entrée de l'utilisateur
        match input.trim().parse() {
            // si le chiffre vaut 1
            Ok(1) => {
                // on renvoie 1
                return 1;
            }
            // si l'entrée n'est pas valide
            _ => {
                // on définit un message d'erreur
                invalid_input_message = "The input is not valid. Please try again.".to_string();
                // on efface l'entrée invalide
                input.clear(); 
                continue;
            }
        }
    }
}