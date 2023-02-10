pub mod dice;

use std::io;
use dice as dicy;


fn main() {
    println!("Please hit return to throw the dice");
    let mut user_input:String = String::new();
    let mut my_dice = dicy::dice::create();
    println!("Dice struct is: {:#?}", my_dice);
    'app_main_loop:loop {
        println!("hit return _> ");
        io::stdin().read_line(&mut user_input).expect("Err reading your input");
        my_dice.throw_dice();
        println!("Dice throw no: {}, Value on face: {}",my_dice.throw_count,my_dice.value);
    }
}
