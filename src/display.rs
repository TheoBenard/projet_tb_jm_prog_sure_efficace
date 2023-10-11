use std::io;
use termion::{color, style};

use crate::{Cell, Minesweeper};
use crate::BOARD_SIZE;
use crate::NUM_MINES;

pub fn print_board(minesweeper_info: &mut Minesweeper) {

    clean_screen();
    
    //Affichage jeu
    println!("   Minesweeper game \n");
    //let white_square = "■";  
    //let empty_square = "□";
    let mut row_count = 0;
    print!("   0 1 2 3 4 5 6 7 11 12 13 14 15 16 17 18 19 20 \n");
    for row in &minesweeper_info.board {
        print!("{}  ", row_count);
        for col in 0..BOARD_SIZE {
            let cell = &row[col];
            if minesweeper_info.game_over || minesweeper_info.revealed.contains(&(row_count, col)) {
                match cell {
                    Cell::Undiscovered => print!(". "),
                    //Cell::Undiscovered => print!(" {} ", white_square),
                    Cell::Number(num) => {
                        // Appliquer des couleurs aux chiffres
                        let colored_number = match num {
                            1 => format!("{}{}1{}", color::Fg(color::Green), style::Bold, style::Reset),
                            2 => format!("{}{}2{}", color::Fg(color::Yellow), style::Bold, style::Reset),
                            3 => format!("{}{}3{}", color::Fg(color::LightRed), style::Bold, style::Reset),
                            4 => format!("{}{}4{}", color::Fg(color::Red), style::Bold, style::Reset),
                            _ => num.to_string(), // Chiffres non colorés
                        };
                        print!("{} ", colored_number);
                    }
                    Cell::Mine => {
                        if minesweeper_info.game_over {
                            print!("\x08\u{1F4A3} ");
                        } else {
                            print!(". "); // on cache les mines non révélées
                            //print!(" {} ", white_square); // on cache les mines non révélées
                        }
                    }
                    Cell::Empty => print!("* "),
                    // Cell::Empty => print!(" {} ", empty_square),
                    Cell::Mark => print!("\u{1F6A9}"),
                }
            } else {
                print!(". ");
                //print!(" {} ", white_square); // on cache les mines non révélées
            }
        }
        if row_count == 2 {
            print!("    Drapeau {}/{} ",minesweeper_info.num_mark, NUM_MINES)
        }
        // TODO : créer une fonction pour centrer ces info par rapport à la grille
        println!();
        row_count += 1;
    }
    println!();
}

pub fn print_menu() -> i32{
    clean_screen();
    println!("  Welcome to the Minesweeper game !");
    println!("Please press any key given below to continue :");
    println!("1 : Minesweeper game rule and How to play.");
    println!("2 : Small 8x8 grid with 10 mines.");
    println!("3 : Medium 16x16 grid with 40 mines.");
    println!("4 : Large 32x32 grid with 160 mines.");
    println!("5 : Xtreme 16x16 grid with 246 mines. (test your luck).");
    println!("Soon : configure your own grid");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Erreur lors de la lecture de l'entrée utilisateur.");

    /*Deletion of spaces*/
    let input = input.trim();
    let input_to_number : i32;

    /*Is the entry a number */
    if input.chars().all(|c| c.is_digit(10)) {
        input_to_number = input.parse().expect("Erreur de conversion en nombre.");
        println!("Vous avez entré le nombre : {}", input_to_number);
    } else {
        println!("L'entrée n'est pas un nombre valide.");
    }
    0
}

fn clean_screen() {
    // Crearing screen with crossterm
    if let Err(err) = crossterm::execute!(
            io::stdout(),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
        ) {
        eprintln!("Erreur lors de l'effacement de l'écran : {}", err);
    }
}