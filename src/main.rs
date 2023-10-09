mod display;

extern crate rand;

use std::collections::HashSet;
use std::io;
use rand::Rng;

//use std::time::{Instant, Duration};
//use std::thread;

const BOARD_SIZE: usize = 5;
const NUM_MINES: usize = 10;
//const FIRST_PLAY: bool = false;

pub struct Minesweeper {
    board: Vec<Vec<Cell>>,
    mines: HashSet<(usize, usize)>,
    revealed: HashSet<(usize, usize)>,
    game_over: bool,
    num_mark: usize,
    first_play: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Mine,
    Number(u8),
    Empty,
    Undiscovered,
    Mark,
}

impl Minesweeper {
    fn new() -> Self {
        let board = vec![vec![Cell::Undiscovered; BOARD_SIZE]; BOARD_SIZE];

        Minesweeper {
            board,
            mines: HashSet::new(),
            revealed: HashSet::new(),
            game_over: false,
            num_mark: 0,
            first_play: false,
        }
    }
    fn check_win(&self) -> bool {
        let num_cells = BOARD_SIZE * BOARD_SIZE;
        let num_revealed = self.revealed.len();
        let num_safe_cells = num_cells - NUM_MINES;

        num_revealed == num_safe_cells
    }

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
            println!("mines around = 0 coordonnées : {:?}, {:?}", row, col);
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

    fn play(&mut self) {
        while !self.game_over && !self.check_win() {
            
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

                    if !self.first_play {
                        // TODO mettre tout ça dans une fonction
                        // Place mines around the first selected cell
                        let mut rng = rand::thread_rng();
                        while self.mines.len() < NUM_MINES {
                            let random_row = rng.gen_range(0..BOARD_SIZE);
                            let random_col = rng.gen_range(0..BOARD_SIZE);
                            
                            let is_adjacent = (random_row as isize - row as isize).abs() <= 1
                            && (random_col as isize - col as isize).abs() <= 1;

                            if !is_adjacent && !self.revealed.contains(&(random_row, random_col)) {
                                self.mines.insert((random_row, random_col));
                                self.board[random_row][random_col] = Cell::Mine;
                            }
                        }
                        
                        for r in row.saturating_sub(1)..=row + 1 {
                            for c in col.saturating_sub(1)..=col + 1 {
                                self.reveal(r as usize, c as usize); 
                            }
                        }

                        self.first_play = true;
                        
                    } else {
                        self.reveal(row as usize, col as usize);
                    }   
                }
            } else if num_chars == 4 && chars[2] == '!' && chars[3] == '\n' { 
                // si l'entrée standard contient 4 caractères
                if let (Some(row), Some(col)) = 
                        (chars[0].to_digit(BOARD_SIZE as u32), 
                        chars[1].to_digit(BOARD_SIZE as u32)) {

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

fn main() {
    println!("Welcome to Minesweeper!\n");
    let mut game = Minesweeper::new();
    game.play();
}

/*
    TIMER à ajouter par la suite

    let mut start_time = Instant::now();

    loop {
        let elapsed_time = start_time.elapsed();
        let seconds = elapsed_time.as_secs();
        let minutes = seconds / 60;
        let remaining_seconds = seconds % 60;

        // Effacez l'écran (pour éviter que le temps précédent ne soit affiché)
        print!("\x1B[2J\x1B[1;1H");

        // Affichez le temps écoulé au format mm:ss
        println!("{:02}:{:02}", minutes, remaining_seconds);

        // Attendez une seconde avant la prochaine mise à jour
        thread::sleep(Duration::from_secs(1));
    }
    
*/