use super::lib::{Player, randint};
use super::board::{Board, BOARD_SIZE};
use super::tile::Tile;


pub struct Bot{
    tile: Tile,
    name: String
}

impl Bot{
    pub fn new(tile: Tile, name: String) -> Self{
        Bot{
            tile,
            name
        }
    }
}

impl Player for Bot{

    fn make_move(&self, board: Board, _cords: Option<(i8, i8)>) -> (i8, i8){
        let mut x: i8;
        let mut y: i8;

        loop{
            x = randint(0, BOARD_SIZE - 1) as i8;
            y = randint(0, BOARD_SIZE - 1) as i8;
            if board.get_tile(x, y) == Tile::Z{
                break;
            }
        }
        
        (x, y)
    }

    fn get_tile(&self) -> Tile{
        self.tile
    }

    fn get_name(&self) -> String{
        self.name.clone()
    }
}