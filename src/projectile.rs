pub struct Projectile {
    pub x: f64,
    pub y: f64,
    pub v_x: f64,
    pub v_y: f64,
    pub mass: f64,
}

impl Projectile {
    pub fn new(angle: f64, speed: f64, mass: f64) -> Self {
        let angle_rad = angle.to_radians();
        Self {
            x: 0.0,
            y: 0.0,
            v_x: speed * angle_rad.cos(),
            v_y: speed * angle_rad.sin(),
            mass,
        }
    }

    pub fn update(&mut self, dt: f64, gravity: f64, drag_coeff: f64) {
        let speed = (self.v_x.powi(2) + self.v_y.powi(2)).sqrt();
        let drag = 0.5 * drag_coeff * speed.powi(2);
        
        let drag_x = if speed > 0.0 { -drag * (self.v_x / speed) } else { 0.0 };
        let drag_y = if speed > 0.0 { -drag * (self.v_y / speed) } else { 0.0 };

        let a_x = drag_x / self.mass;
        let a_y = (drag_y / self.mass) - gravity;

        self.v_x += a_x * dt;
        self.v_y += a_y * dt;
        self.x += self.v_x * dt;
        self.y += self.v_y * dt;
    }
}
// Achievement Grind 1
// Achievement Grind 2
// Achievement Grind 3
// Achievement Grind 4
// Achievement Grind 5
// Achievement Grind 6
// Achievement Grind 7
