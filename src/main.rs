mod map;
mod player;
mod renderer;
mod ray;

use std::time::Instant;

use map::Map;
use player::Player;
use ray::Ray;

use minifb::{Key, Window, WindowOptions};

use crate::renderer::{draw_minimap, draw_scene};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;
const MAP_WIDTH: usize = 10;
const MAP_HEIGHT: usize = 10;

const MINIMAP_SCALE: usize = 6;
const MM_PLAYER_SIZE: usize = MINIMAP_SCALE/3;

const PLAYER_MS: f64 = 5.0;
const PLAYER_RS: f64 = 5.0;

fn main() {
    let map = Map::new();
    let mut player = Player::new();
    // let mut player = Player::new(4.0, 5.0);
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut time;
    let mut frame_time;

    let mut window = Window::new(
        "Raycaster",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);

    println!("Player pos:\t {}, {}", player.pos_x, player.pos_y);
    println!("Map cell [0][0]: {}", map.grid[0][0]);
    println!("Map cell [1][1]: {}", map.grid[1][1]);

    while window.is_open() && !window.is_key_down(Key::Escape) {

        time = Instant::now();
        frame_time = time.elapsed().as_secs_f64() * 100000.0;
        
        
        player.move_player(
            &map,
            window.is_key_down(Key::W),
            window.is_key_down(Key::S),
            window.is_key_down(Key::A),
            window.is_key_down(Key::D),
            window.is_key_down(Key::Left),
            window.is_key_down(Key::Right),
            frame_time);

        for x in 0..MAP_WIDTH{
            //calculate ray pos and dir
            let cam_x: f64 = 2.0 * x as f64 / MAP_WIDTH as f64  - 1.0;
            let ray_dir: (f64,f64) = (player.dir_x as f64 + player.plane_x as f64 * cam_x,
                                     player.dir_y as f64 + player.plane_y as f64 * cam_x);
        }
        
        let rays: Vec<Ray> = (0..WIDTH).map(|x| {
            let camera_x = 2.0 * x as f64 / WIDTH as f64 - 1.0;
            let mut ray = Ray::new(&player, camera_x);
            ray.cast(&map);
            ray
        }).collect();

        buffer.fill(0x00313243);
        draw_scene(&mut buffer, &rays);
        draw_minimap(&mut buffer, &map, &player);
        
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
