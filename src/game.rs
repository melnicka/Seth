use piston_window::types::Color;
use piston_window::*;
use rand::{Rng, rng};

use draw::{draw_block, draw_rectangle};
use snake::{Direction, Snake};

pub mod draw;
pub mod snake;

const FOOD_COLOR: Color = [0.48, 0.03, 0.15, 1.0];
const BORDER_COLOR: Color = [0.0, 0.0, 0.1, 1.0];
const GAMEOVER_COLOR: Color = [0.48, 0.03, 0.15, 1.0];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 2.0;

pub struct Game {
    snake: Snake,

    food_exists: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    pub score: i32,
    game_over: bool,
    waiting_time: f64,
    time_elapsed: f64,
    time_last_eaten: f64
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            time_elapsed: 0.0,
            time_last_eaten: 0.0,
            food_exists: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            score: 0,
            game_over: false,
        }
    }

    fn add_food(&mut self) {
        let mut rng = rng();

        let mut new_x = rng.random_range(1..self.width - 1);
        let mut new_y = rng.random_range(1..self.height - 1);
        // we don't want snake to overlap with the food
        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.random_range(1..self.width - 1);
            new_y = rng.random_range(1..self.height - 1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    fn check_eating(&mut self) -> bool {
        let (head_x, head_y) = self.snake.find_head();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.grow_tail();
            return true;
        }
        return false;
    }

    fn check_if_snake_alive(&mut self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        // if snake's head touches it's body it dies
        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        // checking if snake's head touches the walls
        next_x > 0 && next_x < self.width - 1 && next_y > 0 && next_y < self.height - 1
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.game_over {
            return;
        }

        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            if self.check_eating() {
                self.score += 1;
                self.time_last_eaten = self.time_elapsed;
            }
        } else {
            self.game_over = true
        }
        self.waiting_time = 0.0;
    }
    pub fn time_since_eaten(&self) -> f64 {
        self.time_elapsed - self.time_last_eaten
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 5;
        self.score = 0;
        self.game_over = false;
    }

    pub fn key_pressed(&mut self, key: Key) {
        let dir = match key {
            Key::W => Some(Direction::Up),
            Key::S => Some(Direction::Down),
            Key::A => Some(Direction::Left),
            Key::D => Some(Direction::Right),

            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),

            _ => None,
        };

        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(dir);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }

        // Draw snake first
        self.snake.draw(con, g);

        // Then food
        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }

        // draws only thin borders, not a filled rectangle
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g); // top
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g); // bottom
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g); // left
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g); // right
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;
        self.time_elapsed += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.add_food();
        }
        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }
}
