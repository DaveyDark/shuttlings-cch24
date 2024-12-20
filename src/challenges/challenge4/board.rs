use rand::{rngs::StdRng, Rng};

#[derive(Copy, Clone, Debug, PartialEq)]
enum BoardTile {
    Milk,
    Cookie,
    Empty,
}

impl BoardTile {
    pub fn is_empty(&self) -> bool {
        // Check weather the current tile is empty
        if let BoardTile::Empty = self {
            true
        } else {
            false
        }
    }
    pub fn emoji(&self) -> char {
        match self {
            BoardTile::Cookie => '🍪',
            BoardTile::Milk => '🥛',
            BoardTile::Empty => '⬛',
        }
    }
}

#[derive(Clone, Debug)]
pub struct Board {
    // 4x4 grid of BoardTiles
    board: [[BoardTile; 4]; 4],
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [[BoardTile::Empty; 4]; 4],
        }
    }

    pub fn new_random(rand: &mut StdRng) -> Board {
        let mut board = Board {
            board: [[BoardTile::Empty; 4]; 4],
        };
        for i in 0..4 {
            for j in 0..4 {
                let r = rand.gen::<bool>();
                if r {
                    board.board[i][j] = BoardTile::Cookie;
                } else {
                    board.board[i][j] = BoardTile::Milk;
                }
            }
        }
        board
    }

    pub fn place_tile(&mut self, team: String, col: usize) -> Result<(), String> {
        // Convert from String to BoardTile
        let tile = match team.as_str() {
            "cookie" => BoardTile::Cookie,
            "milk" => BoardTile::Milk,
            _ => return Err("Invalid team".to_string()),
        };

        // Check if col is a valid index
        if col < 1 || col > 4 {
            return Err("Invalid column".to_string());
        }
        let col = col - 1;

        // Try to find empty space in column
        for i in (0..4).rev() {
            if let BoardTile::Empty = self.board[i][col] {
                // Place tile here
                self.board[i][col] = tile;
                return Ok(());
            }
        }

        // Column is full
        return Err("Column full".to_string());
    }

    pub fn get_result(&self) -> Option<String> {
        // Check if we have a winner
        let winner = self.get_winner();
        if winner != BoardTile::Empty {
            return Some(winner.emoji().to_string() + " wins!");
        }

        // See if board is filled
        if self.board.iter().flatten().any(|t| t.is_empty()) {
            None
        } else {
            Some("No winner.".to_string())
        }
    }

    fn get_winner(&self) -> BoardTile {
        for i in 0..4 {
            // Check each column
            let mut win = true;
            for j in 0..3 {
                if self.board[j][i] != self.board[j + 1][i] {
                    win = false;
                }
            }
            if win {
                return self.board[0][i];
            }

            // Check each row
            if self.board[i].windows(2).all(|w| w[0] == w[1]) {
                return self.board[i][0];
            }
        }

        // Check diagonals
        // Top left to bottom right
        if self.board[0][0] == self.board[1][1]
            && self.board[1][1] == self.board[2][2]
            && self.board[2][2] == self.board[3][3]
        {
            return self.board[0][0];
        }
        // Top right to bottom left
        if self.board[0][3] == self.board[1][2]
            && self.board[1][2] == self.board[2][1]
            && self.board[2][1] == self.board[3][0]
        {
            return self.board[0][3];
        }

        BoardTile::Empty
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        // Convert board to string
        let mut str = String::new();
        for row in self.board {
            str.push('⬜');
            for cell in row {
                str.push(cell.emoji())
            }
            str.push('⬜');
            str.push('\n');
        }
        str.push_str("⬜⬜⬜⬜⬜⬜\n");
        if let Some(res) = self.get_result() {
            str.push_str(&res);
            str.push('\n');
        }
        str.to_string()
    }
}
