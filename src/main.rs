mod game_of_life;
mod macros;
mod opengl_ui;
mod rectangle_program;
mod shader;

use std::sync::mpsc::sync_channel;
use std::thread;

use itertools::*;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Game of Life", about = "Conway's Game of Life")]
struct Args {
    #[structopt(short = "h", long = "height", default_value = "200")]
    height: u32,

    #[structopt(short = "w", long = "width", default_value = "200")]
    width: u32,
}

pub fn main() {
    let args = Args::from_args();

    // rendezvous channel for publishing game state
    let (sender, receiver) = sync_channel(0);

    let canvas = opengl_ui::Canvas {
        point_receiver: receiver,
        height: args.height,
        width: args.width,
    };

    thread::spawn(move || {
        let mut game =
            game_of_life::GameOfLife::new_random(args.height as usize, args.width as usize, 0.333);
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

    canvas.run();
}
