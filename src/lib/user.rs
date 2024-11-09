use super::lib::{Player, screen_clear, wait_for_keypress, clear_input_buffer};
use super::board::{Board, BOARD_SIZE};
use super::tile::Tile;
use device_query::Keycode;


pub struct User{
    tile: Tile,
    name: String
}

impl User{
    pub fn new(tile: Tile, name: String) -> Self{
        User{
            tile,
            name
        }
    }
}

impl Player for User{

    fn make_move(&self, board: Board, cords: Option<(i8, i8)>) -> (i8, i8){
        let (mut x, mut y) = if let Some(c) = cords {
            c
        } else {
            (0, 0)
        };

        loop{
            screen_clear();
            board.print_game(Some((x, y)));
            if handle_user_input(&mut x, &mut y) == Ok(()){
                clear_input_buffer();
                screen_clear();
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


fn handle_user_input(x: &mut i8, y: &mut i8) -> Result<(), ()> {
    let k = wait_for_keypress().unwrap();
    match k{
        Keycode::W | Keycode::Up => {
            if *x > 0{
                *x -= 1;
            }
        },
        Keycode::A | Keycode::Left => {
            if *y > 0{
                *y -= 1;
            }
        },
        Keycode::S | Keycode:: Down => {
            if *x < BOARD_SIZE as i8 - 1{
                *x += 1;
            }
        },
        Keycode::D | Keycode::Right=> {
            if *y < BOARD_SIZE as i8 - 1{
                *y += 1;
            }
        },
        Keycode::Enter => {
            return Ok(());
        },
        _ => {
        },
    }

    Err(())
}