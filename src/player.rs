use crate::map::Map;

pub struct Player{
    pub pos: (f64,f64),
    pub vel: (f64,f64),
    pub dir: (f64,f64),
    pub plane: (f64,f64),
    pub vision_angle_rad: f64,
    pub max_speed: f64,
    pub max_rot_speed: f64,
    pub rot_vel: f64,
}

impl Player{
    pub fn new() -> Self{
        Player{
            pos: (4.0,5.0),
            vel: (0.0,0.0),
            dir: (0.0,0.0),
            plane: (0.0,0.0),
            vision_angle_rad: 0.0,
            max_speed: 1.0,
            max_rot_speed: 0.05,
            rot_vel: 0.0
        }

   }
   pub fn check_move_ok(&self, map: &Map, direction: u8)->bool{
      let pos_x = self.pos.0 as f32;
      let pos_y = self.pos.1 as f32;

          match direction{
              0 => { //up (decrease y)
                  if pos_y - 0.2 > 0.0 && map.grid[self.pos.1 as usize - 1][self.pos.0 as usize]
                      == 0{return true;} else{return false;}
              }
              1 => { //down (increase y)
                  if pos_y + 0.2 < 10.0 && map.grid[self.pos.1 as usize + 1][self.pos.0 as usize]
                      == 0{return true;} else{return false;}
              }
              2 => { //left (decrease x)
                  if pos_x - 0.2 > 0.0 && map.grid[self.pos.1 as usize][self.pos.0 as usize - 1]
                      == 0{return true;} else{return false;}
              }
              3 => { //right (increase x)
                  if pos_x - 0.2 < 10.0 && map.grid[self.pos.1 as usize][self.pos.0 as usize + 1]
                      == 0{return true;} else{return false;}
              }
              _ => false,
      }
   }
    pub fn move_player(&mut self, map: &Map, forward: bool, backward: bool, left: bool, right: bool){
        if forward{
            if self.check_move_ok(&map, 0){self.pos.1 -= 0.1;}
            println!("Player position:\t[{},{}]", self.pos.0, self.pos.1);
        }else if backward{
            if self.check_move_ok( &map, 1){self.pos.1 += 0.1;}
            println!("Player position:\t[{},{}]", self.pos.0, self.pos.1);
        }else if left{
            if self.check_move_ok( &map, 2){self.pos.0 -= 0.1;}
            println!("Player position:\t[{},{}]", self.pos.0, self.pos.1);
        }else if right{
            if self.check_move_ok( &map, 3){self.pos.0 += 0.1;}
            println!("Player position:\t[{},{}]", self.pos.0, self.pos.1);
        }
    }
}
