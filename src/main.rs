mod map;
mod player;
mod renderer;

use map::Map;
use player::Player;

use minifb::{Key, Window, WindowOptions};

use crate::renderer::draw_minimap;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;
const MINIMAP_SCALE: usize = 6;
const MAP_WIDTH: usize = 10;
const MAP_HEIGHT: usize = 10;
const MM_PLAYER_SIZE: usize = MINIMAP_SCALE/3;

fn main() {
    let map = Map::new();
    let mut player = Player::new();
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut time = 0;
    let mut old_time = 0;

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

    println!("Player pos:\t {}, {}", player.pos.0, player.pos.1);
    println!("Map cell [0][0]: {}", map.grid[0][0]);
    println!("Map cell [1][1]: {}", map.grid[1][1]);

    while window.is_open() && !window.is_key_down(Key::Escape) {

        player.move_player(
            &map,
           window.is_key_down(Key::W),
           window.is_key_down(Key::S),
           window.is_key_down(Key::A),
           window.is_key_down(Key::D));

        for x in 0..MAP_WIDTH{
            //calculate ray pos and dir
            let cam_x: f64 = 2.0 * x as f64 / MAP_WIDTH as f64  - 1.0;
            let ray_dir: (f64,f64) = (player.dir.0 as f64 + player.plane.0 as f64 * cam_x,
                                     player.dir.1 as f64 + player.plane.1 as f64 * cam_x);
        }


        buffer.fill(0x00e1e1e1);
        draw_minimap(&mut buffer, &map, &player);
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
