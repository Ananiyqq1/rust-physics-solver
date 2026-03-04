pub struct Oscillator {
    pub angle: f64,
    pub angular_velocity: f64,
    pub length: f64,
    pub damping: f64,
}

impl Oscillator {
    pub fn update(&mut self, dt: f64, gravity: f64) {
        let acceleration = -(gravity / self.length) * self.angle.sin() - self.damping * self.angular_velocity;
        self.angular_velocity += acceleration * dt;
        self.angle += self.angular_velocity * dt;
    }
}
