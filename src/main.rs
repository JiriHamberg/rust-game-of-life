mod game_of_life;
mod macros;
mod opengl_ui;
mod shader;

use std::sync::mpsc::sync_channel;
use std::thread;

use itertools::*;

pub fn main() {
    // rendezvous channel for publishing game state
    let (sender, receiver) = sync_channel(0);

    let canvas = opengl_ui::Canvas {
        point_receiver: receiver,
    };

    thread::spawn(move || {
        let mut game = game_of_life::GameOfLife::new_random(200, 200, 0.333);
        loop {
            game.step();

            let points = iproduct!(0..game.get_width(), 0..game.get_height())
                .filter(|(x, y)| match game.get(*x, *y) {
                    Some(true) => true,
                    _ => false,
                })
                .collect();

            // blocks until received or error occurs
            if let Err(_) = sender.send(points) {
                break;
            }
        }
    });

    let canvas_thread = thread::spawn(move || {
        canvas.run();
    });

    canvas_thread.join().unwrap();
}
