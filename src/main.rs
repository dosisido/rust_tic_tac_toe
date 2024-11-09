mod lib{
    pub mod lib;
    pub mod board;
    pub mod tile;
    pub mod user;
    pub mod bot;
}

use lib::lib::{randint, screen_clear, Player};
use lib::board::Board;
use lib::tile::Tile;
use lib::user::User;
use lib::bot::Bot;



fn main() {
    screen_clear();

    let mut board = Board::new();
    let tiles: [Tile; 2] = [Tile::X, Tile::O];
    let i = randint(0, 1);
    let u = User::new(tiles[i], "User".to_string());
    let bot = Bot::new(tiles[1 - i], "Bot".to_string());

    let players: [&dyn Player; 2] = if randint(0, 1) == 0{
        [&u, &bot]
    } else {
        [&bot, &u]
    };
    
    let mut x: i8 = 0;
    let mut y: i8 = 0;
    let mut i = 0;
    let mut game_draw = true;

    for _ in 0..9{
        let current_player = players[i];
        loop{
            (x, y) = current_player.make_move(board,  Some((x, y)));
            let this_tile = current_player.get_tile();
            if board.set_tile(x, y, this_tile) != Ok(()){
                continue;
            };
            break;
        }

        i = (i+1) % 2;
        let won = board.who_won();
        if won.is_ok(){
            screen_clear();
            board.print_game(Some((x, y)));
            let winner = players.iter().filter(|p| p.get_tile() == won.unwrap()).next().unwrap();
            println!("{}({}) won!", winner.get_name(), won.unwrap());
            game_draw = false;
            break;
        }
    }
    if game_draw {
        board.print_game(Some((x, y)));
        println!("It's a draw!");
    }
    
}
