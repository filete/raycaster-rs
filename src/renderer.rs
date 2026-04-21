use crate::{MINIMAP_SCALE, MAP_WIDTH, MAP_HEIGHT, MM_PLAYER_SIZE, WIDTH, HEIGHT};
use crate::map::Map;
use crate::player::Player;
use crate::ray::Ray;

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
        let pixel_x:usize = MAP_WIDTH + player.pos_x as usize * MINIMAP_SCALE;
        let pixel_y:usize = MAP_HEIGHT + player.pos_y as usize * MINIMAP_SCALE;
        buffer[pixel_x + player_pos % MM_PLAYER_SIZE + 2 +
            (pixel_y + player_pos/MM_PLAYER_SIZE + 2) * WIDTH ] = 0x00FFFFFF;
    }
}

pub fn draw_scene(buffer: &mut Vec<u32>, rays: &[Ray]){
    for (x, ray) in rays.iter().enumerate(){
        let mut color: u32 = match ray.hit{
            0 => 0x00000000,
            1 => if ray.side == 1 {0x008C9389} else {0x0090A1A8},
            _ => 0x00000000
        };
        
        // if ray.side == 1 {color/=2;}
        
        let mut line_start = (HEIGHT as i32 / 2 - ray.line_height / 2).max(0) as usize;
        if line_start < 0 {line_start = 0;}
        let mut line_end = (HEIGHT as i32 / 2 + ray.line_height / 2).min(HEIGHT as i32) as usize;
        if line_end >= HEIGHT{line_end = HEIGHT;}
        
       ver_line(x, line_start, line_end, color, buffer); 
    }
}

pub fn ver_line(pos: usize, start: usize, end: usize, color: u32, buffer: &mut Vec<u32>){
   for y in start..end{
       buffer[y*WIDTH+pos] = color;
   } 
}