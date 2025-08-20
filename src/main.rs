extern crate piston_window;
extern crate rand;

use piston_window::types::Color;
use piston_window::*;

use seth::game::Game;
use seth::game::draw::{to_coord, to_coord_u32};

const BACK_COLOR: Color = [0.24, 0.13, 0.21, 1.0];
const TEXT_COLOR: Color = [0.98, 0.96, 0.95, 1.0];

fn main() {
    let (width, height) = (30, 30);

    let mut widnow: PistonWindow =
        WindowSettings::new("Seth", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut glyphs = widnow
    .load_font("font/FiraSans-Bold.ttf").unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = widnow.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        widnow.draw_2d(&event, |c, g, _device| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);

            let score_str = format!("Score:{:?}", game.score);
            let transform = c.transform.trans(to_coord(0), to_coord(1));
            Text::new_color(TEXT_COLOR,20)
            .draw(&score_str, &mut glyphs, &c.draw_state, transform, g).ok();

            let timer_str = format!("Time since eaten: {:.1}s", game.time_since_eaten());
            let transform = c.transform.trans(to_coord(0), to_coord(height));
            Text::new_color(TEXT_COLOR, 20)
            .draw(&timer_str, &mut glyphs, &c.draw_state, transform, g).ok();

            glyphs.factory.encoder.flush(_device);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
