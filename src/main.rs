#![allow(unused_variables)]
#![allow(dead_code)]

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use std::path::Path;
use std::time::Duration;

// #[derive(Debug)]
// struct MyTexture(Texture);

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    players: &Vec<Point>,
    player_textures: &Vec<Texture>,
    sprites: &Vec<Rect>,
    rocks: &mut Vec<(Texture, Rect, Point, Point)>,
    is_shooting: bool,
    // texture: &Texture,
    // position: Point,
    // sprite: Rect,
) -> Result<(), String>
{
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;
    let screen_position = Point::new(width as i32 / 2, height as i32 / 2);
    // canvas.copy(&texture, sprite, screen_rect)?;

    if is_shooting {
        for (i, r) in rocks.iter_mut().enumerate() {
            let texture: &Texture = &r.0;
            let sprite: Rect = r.1;
            let start: Point = r.2;
            let end: Point = r.3;

            if start.x() == width as i32 {
                println!("Done!");
            }

            let rock = Rect::new(start.x(), start.y(), sprite.width(), sprite.height());
            canvas.copy(texture, sprite, rock)?;

            r.2 = start.offset(5, 0);
        }
    }

    // rocks = rocks.filter(|r| r.2 < r.3).collect();

    for (i, p) in players.iter().enumerate() {
        // println!("playres:: {:?}", p)
        let player_position = *p + screen_position;
        let screen_rect =
            Rect::from_center(player_position, sprites[i].width(), sprites[i].height());
        canvas.copy(&player_textures[i], sprites[i], screen_rect)?;
    }

    canvas.present();

    Ok(())
}

fn main() -> Result<(), String>
{
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("Game Tutorial", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();

    let mut players = Vec::<Point>::new();
    let mut sprites = Vec::<Rect>::new();
    let mut textures = Vec::<Texture>::new();

    let mut rocks = Vec::<(Texture, Rect, Point, Point)>::new();
    let player_1: usize = 0;
    let player_2: usize = 1;

    // player 1
    let texture = texture_creator
        .load_texture(Path::new("assets/bardo.png"))
        .unwrap();

    players.push(Point::new(0, 0));
    textures.push(texture);
    sprites.push(Rect::new(0, 0, 26, 36));

    // player 2
    let texture = texture_creator
        .load_texture(Path::new("assets/bardo.png"))
        .unwrap();

    players.push(Point::new(100, 100));
    textures.push(texture);
    sprites.push(Rect::new(0, 108, 26, 36));

    let rock = texture_creator
        .load_texture(Path::new("assets/darkdimension.png"))
        .unwrap();

    // rocks.push((
    // rock,
    // Rect::new(106, 206, 8, 8),
    // Point::new(10, 10),
    // Point::new(10, 10),
    // ));

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    let mut shoot: bool = false;

    'mainloop: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'mainloop;
                }

                // inspect rocks
                Event::KeyDown {
                    keycode: Some(Keycode::I),
                    ..
                } => {
                    // println!("rocks:: {:?}", rocks);
                }
                // shoot
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    println!("Fire!");
                    shoot = true;

                    println!("Fire!, {:?}", shoot);

                    let rock = texture_creator
                        .load_texture(Path::new("assets/darkdimension.png"))
                        .unwrap();

                    let start = players.get(player_1).unwrap();
                    println!("Starting position: {:?}", start);
                    let end = start.offset(20, 0);
                    rocks.push((rock, Rect::new(106, 206, 8, 8), *start, end));
                }

                // player 1
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    let current_position = players.get(player_1).unwrap();
                    let new_position = current_position.offset(-2, 0);
                    std::mem::replace(&mut players[player_1], new_position);
                }

                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    let current_position = players.get(player_1).unwrap();
                    let new_position = current_position.offset(2, 0);
                    std::mem::replace(&mut players[player_1], new_position);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    let current_position = players.get(player_1).unwrap();
                    let new_position = current_position.offset(0, -2);
                    std::mem::replace(&mut players[player_1], new_position);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    let current_position = players.get(player_1).unwrap();
                    let new_position = current_position.offset(0, 2);
                    std::mem::replace(&mut players[player_1], new_position);
                }

                // player 2
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    let current_position = players.get(player_2).unwrap();
                    let new_position = current_position.offset(-2, 0);
                    std::mem::replace(&mut players[player_2], new_position);
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    let current_position = players.get(player_2).unwrap();
                    let new_position = current_position.offset(2, 0);
                    std::mem::replace(&mut players[player_2], new_position);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    let current_position = players.get(player_2).unwrap();
                    let new_position = current_position.offset(0, -2);
                    std::mem::replace(&mut players[player_2], new_position);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    let current_position = players.get(player_2).unwrap();
                    let new_position = current_position.offset(0, 2);
                    std::mem::replace(&mut players[player_2], new_position);
                }

                _ => {}
            }
        }

        // Update
        i = (i + 1) % 255;

        // Render
        render(
            &mut canvas,
            Color::RGB(i, 64, 255 - i),
            &players,
            &textures,
            &sprites,
            &mut rocks,
            shoot,
            // position,
            // sprite,
        )?;

        // Time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
