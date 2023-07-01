use std::fmt;

#[derive(Clone, Copy)]
pub enum Cell {
    X,
    O,
    Empty,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::X => f.write_str("X"),
            Cell::O => f.write_str("O"),
            Cell::Empty => f.write_str(" "),
        }
    }
}

pub enum GameWinner {
    X,
    O,
    Draw,
    Unfinished,
}

impl fmt::Display for GameWinner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameWinner::X => f.write_str("X won"),
            GameWinner::O => f.write_str("O win"),
            GameWinner::Draw => f.write_str("Draw"),
            GameWinner::Unfinished => f.write_str("Game in progress"),
        }
    }
}

pub struct GameBoard {
    pub player: usize,
    field: [Cell; 9],
}

const LINES: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

impl GameBoard {
    pub fn new() -> GameBoard {
        GameBoard {
            player: 0,
            field: [Cell::Empty; 9],
        }
    }

    pub fn is_finished(&self) -> bool {
        !matches!(self.winner(), GameWinner::Unfinished)
    }

    pub fn winner(&self) -> GameWinner {
        let mut empty_cells = false;
        for line in &LINES {
            let mut x_winner = true;
            let mut o_winner = true;
            for &i in line {
                match self.field[i] {
                    Cell::X => o_winner = false,
                    Cell::O => x_winner = false,
                    Cell::Empty => {
                        x_winner = false;
                        o_winner = false;
                        empty_cells = true;
                    }
                }
            }

            if x_winner {
                return GameWinner::X;
            }

            if o_winner {
                return GameWinner::O;
            }
        }

        if empty_cells {
            return GameWinner::Unfinished;
        }

        return GameWinner::Draw;
    }

    pub fn make_move(&mut self, position: usize) -> bool {
        if position >= self.field.len() {
            return false;
        }

        match self.field[position] {
            Cell::Empty => self.field[position] = if self.player == 0 { Cell::X } else { Cell::O },
            _ => return false,
        }

        self.player = (1 - self.player) as usize;

        return true;
    }
}

impl fmt::Display for GameBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::from(" - - - \n");
        for i in 0..3 {
            result.push_str("|");
            for j in 0..3 {
                result.push_str(&format!("{}|", self.field[i * 3 + j]));
            }
            result += "\n - - - \n";
        }

        f.write_str(&result)
    }
}
