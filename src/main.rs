extern crate rand;

use std::collections::HashSet;
use std::io;
use rand::Rng;

const BOARD_SIZE: usize = 8;
const NUM_MINES: usize = 10;

struct Minesweeper {
    board: Vec<Vec<Cell>>,
    mines: HashSet<(usize, usize)>,
    revealed: HashSet<(usize, usize)>,
    game_over: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Mine,
    Number(u8),
    Empty,
    Undiscovered,
}

impl Minesweeper {
    fn new() -> Self {
        let mut board = vec![vec![Cell::Undiscovered; BOARD_SIZE]; BOARD_SIZE];
        let mut mines = HashSet::new();
        let mut rng = rand::thread_rng();

        while mines.len() < NUM_MINES {
            let row = rng.gen_range(0..BOARD_SIZE);
            let col = rng.gen_range(0..BOARD_SIZE);
            mines.insert((row, col));
            board[row][col] = Cell::Mine;
        }

        Minesweeper {
            board,
            mines,
            revealed: HashSet::new(),
            game_over: false,
        }
    }

    fn print_board(&self) {
        // Effacez l'écran en utilisant crossterm
        if let Err(err) = crossterm::execute!(
            io::stdout(),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
        ) {
            eprintln!("Erreur lors de l'effacement de l'écran : {}", err);
        }
    
        // Affichage jeu
        println!("   Minesweeper game \n");
        let mut row_count = 0;
        print!("   0 1 2 3 4 5 6 7\n");
        for row in &self.board {
            print!("{}  ", row_count);
            for col in 0..BOARD_SIZE {
                let cell = &row[col];
                if self.game_over || self.revealed.contains(&(row_count, col)) {
                    match cell {
                        Cell::Mine => print!("X "),
                        Cell::Number(num) => print!("{} ", num),
                        Cell::Empty => print!("* "),
                        Cell::Undiscovered => print!(". "),
                    }
                } else {
                    print!(". "); // Cachez les mines non révélées
                }
                
            }
            println!();
            row_count += 1;
        }
        println!();
        
    }
    
    fn check_win(&self) -> bool {
        let num_cells = BOARD_SIZE * BOARD_SIZE;
        let num_revealed = self.revealed.len();
        let num_safe_cells = num_cells - NUM_MINES;

        num_revealed == num_safe_cells
    }

    fn reveal(&mut self, row: usize, col: usize) {
        if row >= BOARD_SIZE || col >= BOARD_SIZE || self.revealed.contains(&(row, col)) {
            return;
        }

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
            self.print_board();
            println!("Enter row and column (e.g., '3 4') or mark a mine (e.g., '3 3 !'): ");
    
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input.");
    
            let parts: Vec<&str> = input.trim().split_whitespace().collect();

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
        }
    
        if self.game_over {
            self.print_board();
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