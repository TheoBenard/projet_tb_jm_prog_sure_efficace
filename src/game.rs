extern crate rand;

use std::collections::HashSet;
use std::io;
use rand::Rng;

mod display;

const BOARD_SIZE: usize = 20;
const NUM_MINES: usize = 20;

pub struct Minesweeper {
    board: Vec<Vec<Cell>>,
    mines: HashSet<(usize, usize)>,
    pub(crate) revealed: HashSet<(usize, usize)>,
    pub(crate) game_over: bool,
    pub(crate) num_mark: usize,
    pub(crate) player_message: Option<String>,
    first_play : bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cell {
    Mine,
    Number(u8),
    Empty,
    Undiscovered,
    Mark,
}

trait GameActions {
    fn reveal(&mut self, row: usize, col: usize);
    fn mark_mine(&mut self, row: usize, col: usize);
    fn initialize_game_on_first_play(&mut self, row: usize, col: usize);
    fn count_mines_around(&self, row: usize, col: usize) -> u8;
    fn check_win(&mut self) -> bool;
}

impl GameActions for Minesweeper {
    fn reveal(&mut self, row: usize, col: usize) {
        if row >= BOARD_SIZE || col >= BOARD_SIZE {
            self.player_message = Some("La ligne et/ou la colonne ne se trouvent pas dans les limites du jeu...".to_string());
            return;
        }

        if self.revealed.contains(&(row, col)){
            // TODO : lors du premier tour on tombe forcément ici et donc on affiche le message ...
            // self.player_message = Some("Cette case a déjà été révélée...".to_string());
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

    fn mark_mine(&mut self, row: usize, col: usize) {
        println!("mark mine");
        // on sort de la fonction si les coordonnées sont en dehors des limites du tableau
        if row >= BOARD_SIZE || col >= BOARD_SIZE {
            self.player_message = Some("La ligne et/ou la colonne ne se trouvent pas dans les limites du jeu...".to_string());
            return;
        }

        if !self.first_play{
            self.player_message = Some("Vous devez révéler une case...".to_string());
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
        }
    }

    fn initialize_game_on_first_play(&mut self, row: usize, col: usize) {
        if !self.first_play {
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
        }
    }

    fn count_mines_around(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;

        // on parcourt les cases voisines (3x3) autour de la case spécifiée
        for r in row.saturating_sub(1)..=row + 1 {
            for c in col.saturating_sub(1)..=col + 1 {
                if r < BOARD_SIZE && c < BOARD_SIZE && self.mines.contains(&(r, c)) { // on vérifie si la case est à l'intérieur des limites du plateau et contient une mine
                    count += 1; // on incrémente le compteur de mines
                }
            }
        }
        count // on retourne le nombre de mines trouvées autour de la case
    }

    fn check_win(&mut self) -> bool {
        // on calcul la condition de victoire en comparant le nombre de cases révélées et le nombre de cases marquées
        self.revealed.len() - self.num_mark == (BOARD_SIZE * BOARD_SIZE) - NUM_MINES
        // on retourne vrai si le joueur a gagné, sinon on retourne faux
    }
}

// Implémentation du constructeur de la structure Minesweeper
impl Minesweeper {
    pub fn new() -> Self {
        let board = vec![vec![Cell::Undiscovered; BOARD_SIZE]; BOARD_SIZE];

        Minesweeper {
            board,
            mines: HashSet::new(),
            revealed: HashSet::new(),
            game_over: false,
            num_mark: 0,
            player_message: Some("".to_string()),
            first_play: false,
        }
    }
}

pub fn play() {
    // on déclare une nouvelle instance du jeu Minesweeper
    let mut game_instance = Minesweeper::new();

    // on continue tant que le joueur n'a ni perdu ni gagné
    while !game_instance.game_over && !game_instance.check_win() {

        // on affiche le démineur
        display::print_board(&mut game_instance);

        // on explique au joueur comment jouer
        println!("Enter row and column (e.g., '2,0' , '12,19') or mark a mine (e.g., '3,3!' , '5,15!') :");

        // on initialise une nouvelle chaîne modifiable
        let mut input = String::new();

        // on récupère la saisie du joueur
        io::stdin()                                    // on lit l'entrée standard écrit par le joueur
            .read_line(&mut input) // on stocke la chaine mutable dans input
            .expect("Failed to read input.");          // on affiche un message si il y a une erreur

        // Traitement de la saisie utilisateur
        let chars: Vec<char> = input.chars().collect();                             // on extrait chaque caractère de la saisie du joueur dans un vecteur de caractères
        let before_last_chars = chars[chars.len() - 2];                       // on extrait le caractère avant le dernier de la saisie
        let contains_commas = input.contains(',');                            // on vérifie si la saisie contient une virgule
        let num_commas = input.chars().filter(|&c| c == ',').count();  // IA : on compte le nombre de virgules dans la saisie
        let contains_exclamation = input.contains('!');                       // on vérifie si la saisie contient un point d'exclamation
        let input = input.replace("!", "");                        // on supprime les points d'exclamation de la saisie

        if contains_commas && num_commas == 1 { // on vérifie que le joueur a saisi une seule virgule

            let parts: Vec<&str> = input.trim().split(',').collect(); // IA : on divise la saisie en parties distinctes en utilisant la virgule comme séparateur et les stocke dans un vecteur

            // on regarde si le joueur veut découvrir une cellule ou poser un drapeau
            if !contains_exclamation && parts.len() == 2 { // le joueur veut découvrir une cellule

                if let (Some(row),
                        Some(col)) =
                            (parts[0].parse::<usize>().ok(),
                             parts[1].parse::<usize>().ok()) { // IA : on convertit les deux parties de la saisie (ligne et colonne) en valeurs numériques de type `usize`
                    if row < BOARD_SIZE && col < BOARD_SIZE { // on vérifie que ligne et colonne sont compris dans le tableau BOARD_SIZE

                        if !game_instance.first_play { // on vérifie si c'est le premier coup du joueur
                            game_instance.initialize_game_on_first_play(row, col); // si c'est le premier coup, on initialise le jeu en disposant les mines de manière aléatoire
                        } else {
                            game_instance.reveal(row as usize, col as usize); // sinon, on révèle la case du jeu située à la position spécifiée par les valeurs de 'row' et 'col'
                        }
                    } else {
                        // TODO à voir avec cette ligne, j'ai vu un bug en mettant 19,19 ...
                        game_instance.player_message = Some("La ligne et/ou la colonne ne se trouvent pas dans les limites du jeu...".to_string());
                    }
                } else {
                    game_instance.player_message = Some("Veuillez entrer un chiffre ou un nombre conformément aux instructions ci-dessous...".to_string());
                }
            }

            if contains_exclamation && before_last_chars == '!' && parts.len() == 2 { // le joueur veut poser un drapeau

                if let (Some(row),
                        Some(col)) =
                            (parts[0].parse::<usize>().ok(),
                             parts[1].parse::<usize>().ok()) { // IA : on convertit les deux parties de la saisie (ligne et colonne) en valeurs numériques de type `usize`

                    if row < BOARD_SIZE && col < BOARD_SIZE { // on vérifie que ligne et colonne sont compris dans le tableau BOARD_SIZE
                        game_instance.mark_mine(row as usize, col as usize); // on ajoute un drapeau au tableeau du démineur
                    } else {
                        game_instance.player_message = Some("La ligne et/ou la colonne ne se trouvent pas dans les limites du jeu...".to_string());
                    }
                } else {
                    game_instance.player_message = Some("Veuillez entrer un chiffre ou un nombre conformément aux instructions ci-dessous...".to_string());
                }
            }
        } else {
            game_instance.player_message = Some("Vous devez respecter la notation en utilisant une virgule...".to_string());
        }

        if game_instance.game_over { // on vérifie la valeur de game_over (booléen) dans la structure de jeu
            display::print_board(&mut game_instance); // on affiche le tableau de jeu
            println!("Game Over! You hit a mine."); // on informe le joueur
        }

        if game_instance.check_win() { // on vérifie si le joueur a découvert toutes les cases
            println!("Congratulations! You won!"); // on informe le joueur
        }
    }
}

pub fn main_game() {
    let mut is_game_on : i32;
    let mut user_menu_input : i32;
    is_game_on = 1;

    while is_game_on == 1 {
        user_menu_input = display::print_menu();

        if user_menu_input == 1 {
            display::print_game_rule();
        } else if user_menu_input == 9 {
            is_game_on = 0;
        } else {
            play();
        }
    }
}