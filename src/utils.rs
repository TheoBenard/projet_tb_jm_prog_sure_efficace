/*
use crate::{game::Minesweeper};
use crate::BOARD_SIZE;
use crate::NUM_MINES;

pub fn check_win(minesweeper_info: &mut Minesweeper) -> bool {
    let num_cells = BOARD_SIZE * BOARD_SIZE;
    let num_revealed = minesweeper_info.revealed.len();
    let num_safe_cells = num_cells - NUM_MINES;

    num_revealed == num_safe_cells
}

 */