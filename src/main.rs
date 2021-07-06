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

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    players: &Vec<Point>,
    textures: &Vec<Texture>,
    sprites: &Vec<Rect>,
    // texture: &Texture,
    // position: Point,
    // sprite: Rect,
) -> Result<(), String>
{
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;
    // canvas.copy(&texture, sprite, screen_rect)?;

    for (i, p) in players.iter().enumerate() {
        // println!("playres:: {:?}", p)
        let screen_position = *p + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect =
            Rect::from_center(screen_position, sprites[i].width(), sprites[i].height());
        canvas.copy(&textures[i], sprites[i], screen_rect)?;
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

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
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
            // position,
            // sprite,
        )?;

        // Time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
