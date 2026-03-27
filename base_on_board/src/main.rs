mod dice;
use dice::{get_pitch_dice, get_ht_dice, get_c_dice};
use std::io;

fn main() {
    
    loop{
        let mut input_str = String::new();
        io::stdin().read_line(&mut input_str).expect("읽기 실패");
        if input_str.trim() == "q"{
            break;
        }
        get_pitch_dice(input_str.trim());

    }    
}
