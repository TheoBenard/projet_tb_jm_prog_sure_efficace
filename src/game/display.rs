use std::io;
use termion::{color, style};

use crate::game::{Cell, Minesweeper, BOARD_SIZE, NUM_MINES};

pub fn print_board(minesweeper_info: &Minesweeper) {

    clean_screen();

    //Affichage jeu
    println!("   Minesweeper game \n");
    let mut row_count = 0;
    let mut tens_count = 0;
    print!("   ");
    for col in &minesweeper_info.board {
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
    for col in &minesweeper_info.board {
        print!(" {}", row_count);
        if row_count == 9 {
            row_count = 0;
        } else {
            row_count += 1;
        }
    }
    print!("\n");
    row_count = 0;
    for row in &minesweeper_info.board {
        print!("{}  ", row_count);
        if row_count < 10 {
            print!(" ");
        }
        for col in 0..BOARD_SIZE {
            let cell = &row[col];
            if minesweeper_info.game_over || minesweeper_info.revealed.contains(&(row_count, col)) {
                match cell {
                    Cell::Undiscovered => print!(". "),
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
            print!("    Drapeau {}/{} ",minesweeper_info.num_mark, NUM_MINES)
        }
        // TODO : créer une fonction pour centrer ces info par rapport à la grille
        println!();
        row_count += 1;
    }
    // TODO : appel de la fonction qui affiche les erreurs.
    println!();
}

pub fn print_menu() -> i32{
    let mut input_to_number : i32;
    input_to_number = 0;

    while input_to_number == 0 {
        clean_screen();
        println!("  Welcome to the Minesweeper game !");
        println!("Please press any key given below to continue :");
        println!("1 : Minesweeper game rule and How to play.");
        println!("2 : Small 8x8 grid with 10 mines.");
        println!("3 : Medium 16x16 grid with 40 mines.");
        println!("4 : Large 32x32 grid with 160 mines.");
        println!("5 : Xtreme 16x16 grid with 246 mines. (test your luck).");
        println!("Soon : configure your own grid.");
        println!("9 : Quit game.");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Erreur lors de la lecture de l'entrée utilisateur.");

        /*Deletion of spaces*/
        let input = input.trim();

        /*Is the entry a number */
        if input.chars().all(|c| c.is_digit(10)) {
            input_to_number = input.parse().expect("Erreur de conversion en nombre.");
            //println!("Vous avez entré le nombre : {}", input_to_number);
        } else {
            println!("L'entrée n'est pas un nombre valide.");
        }
    }
    input_to_number
}


pub fn print_game_rule() {
    let mut input_to_number : i32;
    input_to_number = 0;
    let mut input_error : i32;
    input_error = 0;

    while input_to_number == 0 {
        clean_screen();
        println!("  Welcome to the game rule of the Minesweeper game !");
        println!("  The goal of this game is to find every mines without clicking on them.");
        println!("  You will be on a grid with undiscovered tiles.");
        println!("  To begin the game you will need to enter the index of a row and then the index of a column. (EG : 1,3). This will be your first move.");
        println!("  This move will uncover tiles and you will see some number appearing.");
        println!("  Those number indicates that there is a mine near the tile where the number is. The tile where the mine is is 'touching' the tiles where the number is.");
        println!("  If you want to uncover a tile where a mine is, then you lose. ");
        println!("  To win, you will need to let uncovered the tile where you think the mines are. ");
        println!("  To ease the things for you, you can mark the mines with a flag where you think the mines are. (Press a row then col with '!' for a flag ( EG : 1,3!))");
        println!("  There is an example below :");
        println!("  * * * ");
        println!("  * 1 * ");
        println!("  * * * ");
        println!("   This '1' indicates that there is 1 mines in is peripherical tiles.");
        println!("  . . . ");
        println!("  . 1 . ");
        println!("  . . \x08\u{1F4A3} ");
        println!("  The mine is in the bellow right corner, so all of the other tiles are empty.");
        println!("  There is an example with a 3 below : ");
        println!("  * * * ");
        println!("  * 3 * ");
        println!("  * * * ");
        println!("   This '1' indicates that there is 1 mines in is peripherical tiles.");
        println!("   \x08\u{1F4A3}. . ");
        println!("  . 1 . ");
        println!("  . \x08\u{1F4A3} \x08\u{1F4A3} ");
        println!("  You are ready, press 1 to get out of this menu.");
        if input_error == 1 {
            println!("You entered a wrong value.");
            input_error = 0;
        }

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Erreur lors de la lecture de l'entrée utilisateur.");
        println!("{}", input);

        /*Deletion of spaces*/
        let input = input.trim();

        /*Is the entry a number */
        if input.chars().all(|c| c.is_digit(10)) {
            input_to_number = input.parse().expect("Erreur de conversion en nombre.");
            if input_to_number != 1 {
                input_error = 1;
                input_to_number = 0;
            }
        } else {
            input_error = 1;
        }
    }
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