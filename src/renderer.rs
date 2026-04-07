use crate::{MINIMAP_SCALE, MAP_WIDTH, MAP_HEIGHT, MM_PLAYER_SIZE, WIDTH};
use crate::map::Map;
use crate::player::Player;

pub fn draw_minimap(buffer: &mut Vec<u32>, map: &Map, player: &Player ) {
    for row in 0..MAP_WIDTH{
        for col in 0..MAP_HEIGHT{
            let color: u32 = if map.grid[row][col] == 1 {0x00453B46} else {0x0090A1A8};
            for pixel_y in 0..MINIMAP_SCALE{
                for pixel_x in 0..MINIMAP_SCALE{
                    let x = MAP_WIDTH + col * MINIMAP_SCALE + pixel_x;
                    let y = MAP_HEIGHT + row * MINIMAP_SCALE + pixel_y;
                    buffer[y*WIDTH+x] = color;
                }
            }
        }
    }
    for player_pos in 0..MM_PLAYER_SIZE*MM_PLAYER_SIZE{
        let pixel_x:usize = MAP_WIDTH + player.pos.0 as usize * MINIMAP_SCALE;
        let pixel_y:usize = MAP_HEIGHT + player.pos.1 as usize * MINIMAP_SCALE;
        buffer[pixel_x + player_pos % MM_PLAYER_SIZE + 2 +
            (pixel_y + player_pos/MM_PLAYER_SIZE + 2) * WIDTH ] = 0x00C93749;
    }
}
