extern crate rand;

use std::collections::HashSet;
use std::io;
use rand::Rng;

use crate::{Cell, display, utils};
use crate::BOARD_SIZE;
use crate::NUM_MINES;

pub struct Minesweeper {
    pub(crate) board: Vec<Vec<Cell>>,
    mines: HashSet<(usize, usize)>,
    pub(crate) revealed: HashSet<(usize, usize)>,
    pub(crate) game_over: bool,
    pub(crate) num_mark: usize,
}

trait GameActions {
    fn reveal(&mut self, row: usize, col: usize);
    fn mark_mine(&mut self, row: usize, col: usize);
}

impl GameActions for Minesweeper {
    fn reveal(&mut self, row: usize, col: usize) {
        if row >= BOARD_SIZE || col >= BOARD_SIZE {
            return;
        }

        if self.revealed.contains(&(row, col)){
            // TODO : modifier la variable d'erreur qui sera affiché au joueur
            return;
        }

        // TODO : créer une fonction "check_game_over" comme pour "check_win" ?
        if self.mines.contains(&(row, col)) {
            self.game_over = true;
            return;
        }

        let mines_around = self.count_mines_around(row, col);
        self.revealed.insert((row, col));

        self.board[row][col] = match mines_around {
            0 => Cell::Empty,
            n => Cell::Number(n),
        };

        if mines_around == 0 {
            for r in row.saturating_sub(1)..=row + 1 {
                for c in col.saturating_sub(1)..=col + 1 {
                    self.reveal(r, c);
                }
            }
        }
    }

    fn mark_mine(&mut self, row: usize, col: usize) {
        // on sort de la fonction si les coordonnées sont en dehors des limites du tableau
        if row >= BOARD_SIZE || col >= BOARD_SIZE {
            return;
        }

        if self.revealed.contains(&(row, col)) && self.board[row][col] == Cell::Mark {

            match self.board[row][col] {
                Cell::Mark => {
                    self.board[row][col] = Cell::Mine; // on change la cellule d'état
                    self.revealed.remove(&(row, col)); // on retire le drapeau du tableau
                    self.num_mark -= 1;
                },
                _ => {
                    // on ne fait rien si la cellule contient autre chose qu'un drapeau
                },
            }
        } else if self.num_mark < NUM_MINES && self.board[row][col] == Cell::Undiscovered || self.board[row][col] == Cell::Mine {

            match self.board[row][col] {
                Cell::Undiscovered | Cell::Mine => {
                    self.board[row][col] = Cell::Mark; // on change la cellule d'état
                    self.revealed.insert((row, col)); // on ajoute le drapeau au tableau
                    self.num_mark += 1;
                },
                _ => {
                    // on ne fait rien si la cellule contient autre chose que rien ou une mine
                },
            }
        } else {
            println!("Cellule déjà occupée...");
            // TODO : gérer les messages d'erreur (avec une varibale par exemple)
        }
    }
}

impl Minesweeper {
    pub fn new() -> Self {
        let mut board = vec![vec![Cell::Undiscovered; BOARD_SIZE]; BOARD_SIZE];
        let mut mines = HashSet::new();
        let mut rng = rand::thread_rng();

        while mines.len() < NUM_MINES {
            let row = rng.gen_range(0..BOARD_SIZE);
            let col = rng.gen_range(0..BOARD_SIZE);
            if !mines.contains(&(row, col)) {
                mines.insert((row, col));
                board[row][col] = Cell::Mine;
            }
        }

        Minesweeper {
            board,
            mines,
            revealed: HashSet::new(),
            game_over: false,
            num_mark: 0,
        }
    }


    fn count_mines_around(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;

        for r in row.saturating_sub(1)..=row + 1 {
            for c in col.saturating_sub(1)..=col + 1 {
                if r < BOARD_SIZE && c < BOARD_SIZE && self.mines.contains(&(r, c)) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn play(&mut self) {
        while !self.game_over && !utils::check_win(self) {

            // on affiche le démineur
            display::print_board(self);

            // on récupère la saisie du joueur
            println!("Enter row and column (e.g., '24') or mark a mine (e.g., '33!') :");
            let mut input = String::new(); // on initialise une nouvelle chaîne modifiable
            io::stdin() // on lit l'entrée standard écrit par le joueur
                .read_line(&mut input) // on stocke la chaine dans input
                .expect("Failed to read input."); // on affiche un message si problème

            let chars: Vec<char> = input.chars().collect(); // on récupère chaque caractère dans un vecteur
            let num_chars = chars.len(); // on compte le nombre de caractère dans "input"

            //println!("chars : {:?}", chars);
            //println!("num_chars : {}", num_chars);

            if num_chars == 3 && chars[2] == '\n' {
                // si l'entrée standard contient 3 caractères
                if let (Some(row), Some(col)) =
                    (chars[0].to_digit(BOARD_SIZE as u32),
                     chars[1].to_digit(BOARD_SIZE as u32)) {

                    //println!("first : {}", row);
                    //println!("second : {}", col);
                    self.reveal(row as usize, col as usize)
                }
            } else if num_chars == 4 && chars[2] == '!' && chars[3] == '\n' {
                // si l'entrée standard contient 4 caractères
                if let (Some(row), Some(col)) =
                    (chars[0].to_digit(BOARD_SIZE as u32),
                     chars[1].to_digit(BOARD_SIZE as u32)) {

                    //println!("first : {}", row);
                    //println!("second : {}", col);
                    self.mark_mine(row as usize, col as usize);
                }
            } else {
                println!("Error, invalid input. Try again");
                // TODO : créer une variable "globale" qui stockera et affichera toutes les erreurs
                // TODO : si le joueur choisit une cellule déjà dévoilée
            }
            /*
                println!("________________________________________");
                let parts: Vec<&str> = input.trim().split_whitespace().collect();
                println!("{:?}", input);
                if parts.len() == 3 && parts[2] == "!" {
                    // The player is marking a mine
                    let row: usize = parts[0]
                        .parse()
                        .expect("Invalid input for row.");
                    let col: usize = parts[1]
                        .parse()
                        .expect("Invalid input for column.");

                    if row < BOARD_SIZE && col < BOARD_SIZE {
                        self.board[row][col] = Cell::Mine;
                    } else {
                        println!("Invalid row or column.");
                    }
                } else if parts.len() == 2 {
                    // The player is revealing a cell
                    let row: usize = parts[0]
                        .parse()
                        .expect("Invalid input for row.");
                    let col: usize = parts[1]
                        .parse()
                        .expect("Invalid input for column.");

                    self.reveal(row, col);
                } else {
                    println!("Invalid input.");
                }
             */
        }

        if self.game_over {
            display::print_board(self);
            println!("Game Over! You hit a mine.");
        } else {
            println!("Congratulations! You won!");
        }
    }
}