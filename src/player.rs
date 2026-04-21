use crate::map::Map;
use crate::{PLAYER_MS, PLAYER_RS};

pub struct Player{
    pub pos_x: f64,
    pub pos_y: f64,
    pub vel: (f64,f64),
    pub dir_x: f64,
    pub dir_y: f64,
    pub plane_x: f64,
    pub plane_y: f64,
    pub vision_angle_rad: f64,
    pub max_speed: f64,
    pub max_rot_speed: f64,
    pub rot_vel: f64,
}

impl Player{
    pub fn new() -> Self{
        Player{
            pos_x: 4.0,
            pos_y: 5.0,
            vel: (0.0,0.0),
            dir_x: 0.0,
            dir_y: -1.0,
            plane_x: 0.66,
            plane_y: 0.,
            vision_angle_rad: 0.0,
            max_speed: 1.0,
            max_rot_speed: 0.05,
            rot_vel: 0.0
        }

   }
    pub fn move_player(&mut self, map: &Map, forward: bool, backward: bool, left: bool, right: bool, left_rot: bool, right_rot: bool, frame_time: f64){
        let move_speed = frame_time * PLAYER_MS;
        let rot_speed = frame_time * PLAYER_RS;
        
        if forward{
            if map.grid[self.pos_y as usize][(self.pos_x + self.dir_x * move_speed) as usize] == 0 {self.pos_x += self.dir_x * move_speed}
            if map.grid[(self.pos_y + self.dir_y * move_speed) as usize][self.pos_x as usize] == 0 {self.pos_y += self.dir_y * move_speed}
        }
        
        if backward{
            if map.grid[self.pos_y as usize][(self.pos_x - self.dir_x * move_speed) as usize] == 0 {self.pos_x -= self.dir_x * move_speed}
            if map.grid[(self.pos_y - self.dir_y * move_speed) as usize][self.pos_x as usize] == 0 {self.pos_y -= self.dir_y * move_speed}
        }
        
        if left{
            if map.grid[self.pos_y as usize][(self.pos_x + self.dir_y * move_speed) as usize] == 0 {self.pos_x += self.dir_y * move_speed}
            if map.grid[(self.pos_y + self.dir_x * move_speed) as usize][self.pos_x as usize] == 0 {self.pos_y += self.dir_x * move_speed}
        }
        
        if right{
            if map.grid[self.pos_y as usize][(self.pos_x - self.dir_y * move_speed) as usize] == 0 {self.pos_x -= self.dir_y * move_speed}
            if map.grid[(self.pos_y - self.dir_x * move_speed) as usize][self.pos_x as usize] == 0 {self.pos_y -= self.dir_x * move_speed}
        }
        
        if  left_rot{
            let old_dir_x = self.dir_x;
            self.dir_x = self.dir_x * (-rot_speed).cos() - self.dir_y * (-rot_speed).sin();
            self.dir_y = old_dir_x * (-rot_speed).sin() + self.dir_y * (-rot_speed).cos();
            let old_plane_x = self.plane_x;
            self.plane_x = self.plane_x * (-rot_speed).cos() - self.plane_y * (-rot_speed).sin();
            self.plane_y = old_plane_x * (-rot_speed).sin() + self.plane_y * (-rot_speed).cos();
        }
        
        if  right_rot{
            let old_dir_x = self.dir_x;
            self.dir_x = self.dir_x * rot_speed.cos() - self.dir_y * rot_speed.sin();
            self.dir_y = old_dir_x * rot_speed.sin() + self.dir_y * rot_speed.cos();
            let old_plane_x = self.plane_x;
            self.plane_x = self.plane_x * rot_speed.cos() - self.plane_y * rot_speed.sin();
            self.plane_y = old_plane_x * rot_speed.sin() + self.plane_y * rot_speed.cos();
        }
        
        //println!("Player position:\t[{},{}]", self.pos_x, self.pos_y);
    }
}
