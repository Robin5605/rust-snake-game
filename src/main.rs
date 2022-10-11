extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;
use rand::Rng;

#[derive(PartialEq)]
enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

const WINDOW_SIZE: u32 = 800;
const GRID_SIZE: u32 = 20;

fn check_collision(rect1: &Rect, rect2: &Rect) -> bool {
    rect1.x() == rect2.x() && rect1.y() == rect2.y()
}

fn check_edge_collision(rect1: &Rect) -> bool {
    rect1.x() == (WINDOW_SIZE as i32) - (rect1.width() as i32) ||
    rect1.y() == (WINDOW_SIZE as i32) - (rect1.height() as i32)
}

fn gen_fruit() -> Rect {
    let x = rand::thread_rng().gen_range(0i32..(WINDOW_SIZE as i32)) + 1;
    let y = rand::thread_rng().gen_range(0i32..(WINDOW_SIZE as i32)) + 1;
    let grid_size_i32 = GRID_SIZE as i32;
    
    Rect::new(x - (x % grid_size_i32), y - (y % grid_size_i32), GRID_SIZE, GRID_SIZE)
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();


    let window = video_subsystem.window("Snake", WINDOW_SIZE, WINDOW_SIZE)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();


    let mut snake_body: Vec<Rect> = vec![Rect::new(0, 0, GRID_SIZE, GRID_SIZE)];
    let mut fruit_rect = gen_fruit();

    let mut direction = Direction::RIGHT;
    'running: loop {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(kc), .. } => {
                    direction = match kc {
                        Keycode::W => { 
                            if direction == Direction::DOWN {
                                direction
                            } else {
                                Direction::UP
                            }
                        },
                        Keycode::A => { 
                            if direction == Direction::RIGHT {
                                direction
                            } else {
                                Direction::LEFT
                            }
                        },
                        Keycode::S => { 
                            if direction == Direction::UP {
                                direction
                            } else {
                                Direction::DOWN
                            }
                        },
                        Keycode::D => { 
                            if direction == Direction::LEFT {
                                direction
                            } else {
                                Direction::RIGHT
                            }
                        },
                        _ => { direction }
                    }
                },
                _ => {}
            }
        }

        // Draw fruit
        canvas.set_draw_color(Color::GREEN);
        canvas.fill_rect(fruit_rect).unwrap();


        let grid_size_i32 = GRID_SIZE as i32;
        match snake_body.last().cloned() {
            None => {},
            Some(head_rect) => {

                // Tail collision
                for chunk in &snake_body[0..snake_body.len() - 1] {
                    if check_collision(chunk, &head_rect) {
                        break 'running;
                    }
                }

                // Edge collision
                if check_edge_collision(&head_rect) {
                    break 'running;
                }

                // Turning
                let new_rect = match direction {
                    Direction::UP => Rect::new(head_rect.x(), head_rect.y() - grid_size_i32, GRID_SIZE, GRID_SIZE),
                    Direction::DOWN => Rect::new(head_rect.x(), head_rect.y() + grid_size_i32, GRID_SIZE, GRID_SIZE),
                    Direction::LEFT => Rect::new(head_rect.x() - grid_size_i32, head_rect.y(), GRID_SIZE, GRID_SIZE),
                    Direction::RIGHT => Rect::new(head_rect.x() + grid_size_i32, head_rect.y(), GRID_SIZE, GRID_SIZE),
                };
                snake_body.push(new_rect);

                // Fruit collision check
                if check_collision(&head_rect, &fruit_rect) {
                    fruit_rect = gen_fruit();
                } else {
                    snake_body.remove(0);
                }
            }
        }

        // Draw body
        canvas.set_draw_color(Color::GREEN);
        canvas.fill_rects(&snake_body).unwrap();
        

        canvas.present();
         ::std::thread::sleep(Duration::new(0, 100_000_000));
    }
}