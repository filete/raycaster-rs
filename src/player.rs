pub struct Player{
    pub vision_angle_rad: f64,
    pub max_speed: f64,
    pub max_rot_speed: f64,
    pub x_pos: f64,
    pub y_pos: f64,
    pub x_vel: f64,
    pub y_vel: f64,
    pub rot_vel: f64,
}

impl Player{
    pub fn new() -> Self{
        Player{
            vision_angle_rad: 0.0,
            max_speed: 1.0,
            max_rot_speed: 0.05,
            x_pos: 5.0,
            y_pos: 9.0,
            x_vel: 0.0,
            y_vel: 0.0,
            rot_vel: 0.0
        }
    }
}
