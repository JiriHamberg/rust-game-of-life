use super::game_of_life;
use std::{thread, time};

pub fn run() {
    let mut game = game_of_life::GameOfLife::new_random(50, 50, 0.25);

    loop {
        println!("{}", game);
        thread::sleep(time::Duration::from_millis(40));
        game.step();
    }
}
