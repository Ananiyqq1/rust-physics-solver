pub struct Body {
    pub pos: (f64, f64),
    pub vel: (f64, f64),
    pub mass: f64,
}

pub fn calculate_gravity(b1: &Body, b2: &Body, g: f64) -> (f64, f64) {
    let dx = b2.pos.0 - b1.pos.0;
    let dy = b2.pos.1 - b1.pos.1;
    let dist_sq = dx * dx + dy * dy + 1e-9; // Softening factor
    let dist = dist_sq.sqrt();
    let force = (g * b1.mass * b2.mass) / dist_sq;
    
    (force * dx / dist, force * dy / dist)
}
