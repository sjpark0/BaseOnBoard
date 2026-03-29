mod dice;
mod process;

use dice::{get_pitch_dice, get_ht_dice, get_c_dice};
use process::Games;

use std::io;

fn main() {
    let mut game = Games::new();
        
    loop{
        let mut input_str = String::new();
        io::stdin().read_line(&mut input_str).expect("읽기 실패");
        if input_str.trim() == "q"{
            break;
        }
        if game.set_process(input_str.trim()) == true{
            break;
        }
    }    
    game.print();
}
