use device_query::{DeviceQuery, DeviceState, Keycode};
use std::thread::sleep;
use std::time::Duration;
use super::board::Board;
use super::tile::Tile;
use rand::Rng;
use std::io::{self, Write};

pub fn screen_clear(){
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
pub fn clear_input_buffer() {
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();
}

// pub fn wait_for_keypress() -> Option<Keycode> {
//     let device_state = DeviceState::new();
//     let mut last_keys = vec![];

//     // Poll until a single key is pressed and then released, then return that key
//     loop {
//         let keys: Vec<Keycode> = device_state.get_keys();

//         // Check if any key is pressed
//         if !keys.is_empty() {
//             last_keys = keys;
//         } else if !last_keys.is_empty() {
//             // Return the first key in the list when all keys are released
//             return Some(last_keys[0].clone());
//         }

//         // Sleep briefly to avoid high CPU usage
//         sleep(Duration::from_millis(50));
//     }
// }

use crossterm::terminal::{enable_raw_mode, disable_raw_mode};


pub fn wait_for_keypress() -> Option<Keycode> {
    // Enable raw mode at the start to prevent echoing characters
    if enable_raw_mode().is_err() {
        return None; // Return None if enabling raw mode fails
    }
    
    let device_state = DeviceState::new();
    let mut last_keys = vec![];

    // Poll until a single key is pressed and then released, then return that key
    loop {
        let keys: Vec<Keycode> = device_state.get_keys();

        // Check if any key is pressed
        if !keys.is_empty() {
            last_keys = keys;
        } else if !last_keys.is_empty() {
            // Disable raw mode before returning to restore terminal behavior
            disable_raw_mode().ok();
            return Some(last_keys[0].clone());
        }

        // Sleep briefly to avoid high CPU usage
        sleep(Duration::from_millis(50));
    }
}

pub fn randint(min: usize, max: usize) -> usize {
    rand::thread_rng().gen_range(min..=max)
}

pub trait Player{
    fn make_move(&self, b: Board, cords: Option<(i8, i8)>) -> (i8, i8);
    fn get_tile(&self) -> Tile;
    fn get_name(&self) -> String;
}