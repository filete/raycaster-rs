use crate::{HEIGHT, MAP_HEIGHT, MAP_WIDTH};
use crate::player::Player;
use crate::map::Map;

const MAX_STEPS: usize = MAP_WIDTH + MAP_HEIGHT;

pub struct Ray{
    // Ray direction
    pub ray_dir_x: f64,
    pub ray_dir_y: f64,
    // Delta distances
    pub delta_dist_x: f64,
    pub delta_dist_y: f64,
    // Position on the map
    pub map_pos_x: i16,
    pub map_pos_y: i16,
    // Length of ray to the next x/y
    pub side_dist_x: f64,
    pub side_dist_y: f64,
    // Step direction
    pub step_dir_x: i8, 
    pub step_dir_y: i8, 
    
    pub hit: u8, // Type of wall hit (0 for no hit, 1 for wall)
    pub side: u8, // 0 for x-side hit, 1 for y-side hit
    
    pub perp_wall_dist: f64, // Perpendicular distance from the camera plane to the hit 
    pub line_height: i32,
}

impl Ray{
    pub fn new(player: &Player, camera_x: f64) -> Self{
        let ray_dir_x =  player.dir_x + player.plane_x * camera_x;
        let ray_dir_y =  player.dir_y + player.plane_y * camera_x;
        
        let delta_dist_x = if ray_dir_x == 0.0 {f64::INFINITY} else {(1.0/ray_dir_x).abs()};
        let delta_dist_y = if ray_dir_y == 0.0 {f64::INFINITY} else {(1.0/ray_dir_y).abs()};
        
        let map_pos_x = player.pos_x.floor() as i16;
        let map_pos_y = player.pos_y.floor() as i16;
        
        let side_dist_x: f64;
        let side_dist_y: f64;
        let step_dir_x : i8;
        let step_dir_y : i8;
        
        if ray_dir_x > 0.0 {
            side_dist_x = ( map_pos_x as f64 + 1.0 - player.pos_x) * delta_dist_x;
            step_dir_x = 1;
        } else {
            side_dist_x = (player.pos_x - map_pos_x as f64) * delta_dist_x;
            step_dir_x = -1;
        }
        
        if ray_dir_y > 0.0 {
            side_dist_y = (map_pos_y as f64 + 1.0 - player.pos_y) * delta_dist_y;
            step_dir_y = 1;
        } else {
            side_dist_y = (player.pos_y - map_pos_y as f64) * delta_dist_y;
            step_dir_y = -1;
        }
        
        Ray{
            ray_dir_x,
            ray_dir_y,
            delta_dist_x,
            delta_dist_y,
            map_pos_x,
            map_pos_y,
            side_dist_x,
            side_dist_y,
            step_dir_x,
            step_dir_y,
            hit: 0,
            side: 0,
            perp_wall_dist: 0.0,
            line_height: 0,
        }
    }
    
    pub fn cast(&mut self, map: &Map){
        let mut steps = 0;
        while self.hit == 0 && steps <= MAX_STEPS{
            if self.side_dist_x > self.side_dist_y{
                self.side_dist_y += self.delta_dist_y;
                self.map_pos_y += self.step_dir_y as i16;
                self.side = 1;
            }else{
                self.side_dist_x += self.delta_dist_x;
                self.map_pos_x += self.step_dir_x as i16;
                self.side = 0;
            }
            steps += 1;
            if map.grid[self.map_pos_y as usize][self.map_pos_x as usize] > 0 { self.hit = 1;}
        }
        if self.side == 0{self.perp_wall_dist = self.side_dist_x - self.delta_dist_x;}
        else{ self.perp_wall_dist = self.side_dist_y - self.delta_dist_y; }
        
        if self.perp_wall_dist > 0.0 {
            self.line_height = (HEIGHT as f64 / self.perp_wall_dist) as i32;
        } else {
            self.line_height = 0;
        }
    }
}