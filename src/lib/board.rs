pub const BOARD_SIZE: usize = 3;
use super::tile::Tile;

#[derive(Copy, Clone)]
pub struct Board{
    board: [[Tile; BOARD_SIZE]; BOARD_SIZE],
}


impl Board{
    pub fn new() -> Self{
        Board{
            board: [[Tile::Z; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    #[allow(dead_code)]
    pub fn from_tile(t:Tile) -> Self{
        Board{
            board: [[t; 3]; 3],
        }
    }

    pub fn print_game(&self, cords: Option<(i8, i8)>){
        for (row_index, row) in self.board.iter().enumerate(){
            for (column_index, tile) in row.iter().enumerate(){
                if cords.is_some(){
                    let (x, y) = cords.unwrap();
                    if row_index == x as usize && column_index == y as usize{
                        print!("[{}] ", tile);
                    }else{
                        print!(" {}  ", tile);
                    }
                }
            }
            println!();
        }
    }

    pub fn set_tile(&mut self, x: i8, y: i8, tile: Tile) -> Result<(), ()>{
        if self.board[x as usize][y as usize] == Tile::Z{
            self.board[x as usize][y as usize] = tile;
            Ok(())
        }else{
            Err(())
        }
    }

    pub fn get_tile(&self, x: i8, y: i8) -> Tile{
        self.board[x as usize][y as usize]
    }

    pub fn who_won(&self) -> Result<Tile, ()>{
        for row in &self.board{
            if row[0] == row[1] && row[1] == row[2] && row[0] != Tile::Z{
                return Ok(row[0]);
            }
        }

        for i in 0..BOARD_SIZE{
            if self.board[0][i] == self.board[1][i] && self.board[1][i] == self.board[2][i] && self.board[0][i] != Tile::Z{
                return Ok(self.board[0][i]);
            }
        }

        if self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2] && self.board[0][0] != Tile::Z{
            return Ok(self.board[0][0]);
        }

        if self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0] && self.board[0][2] != Tile::Z{
            return Ok(self.board[0][2]);
        }

        Err(())
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.board {
            for tile in row {
                write!(f, "{} ", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}