mod game_of_life;
mod macros;
mod opengl_ui;
mod shader;
mod shell_ui;

use std::sync::mpsc::sync_channel;
use std::thread;

use itertools::*;

pub fn main() {
    let (sender, receiver) = sync_channel(0);

    let canvas = opengl_ui::Canvas {
        point_receiver: receiver,
    };

    let game_thread = thread::spawn(move || {
        let mut game = game_of_life::GameOfLife::new_random(200, 200, 0.333);
        loop {
            game.step();

            let points = iproduct!(0..game.get_width(), 0..game.get_height())
                .filter(|(x, y)| match game.get(*x, *y) {
                    Some(true) => true,
                    _ => false,
                })
                .collect();

            sender.send(points).unwrap();
        }
    });

    let canvas_thread = thread::spawn(move || {
        canvas.run();
    });

    game_thread.join().unwrap();
    canvas_thread.join().unwrap();
}
