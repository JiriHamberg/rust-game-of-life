mod game_of_life;
mod macros;
mod opengl_ui;
mod shader;
mod shell_ui;

pub fn main() {
    //shell_ui::run();
    //opengl_ui::run();

    let canvas = opengl_ui::Canvas {
        points: vec![(0, 0), (-1, 0), (1, 1), (1, 2), (1, 3)],
    };

    canvas.run();
}
