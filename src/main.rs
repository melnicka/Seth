extern crate rand;
extern crate piston_window;

use piston_window::*;
use piston_window::types::Color;

use seth::game::Game;
use seth::game::draw::to_coord_u32;

const BACK_COLOR: Color = [0.24, 0.13, 0.21, 1.0];

fn main() {
    let (width, height) = (30,30);

    let mut widnow: PistonWindow = WindowSettings::new(
        "Seth",
        [to_coord_u32(width), to_coord_u32(height)]
    ).exit_on_esc(true).build().unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = widnow.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        widnow.draw_2d(&event, |c, g, _device| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
