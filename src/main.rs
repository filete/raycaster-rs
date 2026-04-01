mod map;
mod player;

use map::Map;
use player::Player;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

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
    let mut map = Map::new();
    let mut player = Player::new();

    println!("Player pos:\t {}, {}", player.x_pos, player.y_pos);
    println!("Map cell [0][0]: {}", map.grid[0][0]);
    println!("Map cell [1][1]: {}", map.grid[1][1]);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // for i in buffer.iter_mut() {
        //     *i = 0; // write something more funny here!
        // }

        buffer.fill(0x00f1f1f1);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
