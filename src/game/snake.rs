use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

use super::draw::draw_block;

const SNAKE_COLOR: Color = [0.98, 0.96, 0.95, 1.0]; // R, G, B, opacity

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    pub fn opposite(&self) -> Direction {
        // if, for example, the snake is going up and you try to go down, it won't let you
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();

        // at the start of the game snake is three blocks long
        body.push_back(Block { x: x + 2, y: y });
        body.push_back(Block { x: x + 1, y: y });
        body.push_back(Block { x: x, y: y });

        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g)
        }
    }

    pub fn find_head(&self) -> (i32, i32) {
        let head = self.body.front().unwrap(); // unwrap gets rid of the Option enum
        (head.x, head.y)
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }

        let (head_x, head_y): (i32, i32) = self.find_head();

        let new_block = match self.direction {
            Direction::Up => Block {
                x: head_x,
                y: head_y - 1,
            },
            Direction::Down => Block {
                x: head_x,
                y: head_y + 1,
            },
            Direction::Left => Block {
                x: head_x - 1,
                y: head_y,
            },
            Direction::Right => Block {
                x: head_x + 1,
                y: head_y,
            },
        };

        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        // predicting where the snake’s head will be after the next move
        let (head_x, head_y) = self.find_head();

        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => {}
        }

        match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }

    pub fn grow_tail(&mut self) {
        let tail = self.tail.clone().unwrap();
        self.body.push_back(tail);
    }

    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        // checking if a block overlaps with any part of snake's body exept the head
        let mut i = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }
            i += 1;
            if i == self.body.len() - 1 {
                // -1 because we don't include the head
                break;
            }
        }
        return false;
    }
}
