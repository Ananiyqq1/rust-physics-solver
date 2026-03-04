use physics_solver::projectile::Projectile;
use std::thread;
use std::time::Duration;

fn main() {
    println!("--- Creative Physics Solver ---");
    println!("Simulating Projectile Motion with Air Resistance...");

    let mut p = Projectile::new(45.0, 50.0, 1.0);
    let dt = 0.1;
    let mut time = 0.0;

    while p.y >= 0.0 {
        p.update(dt, 9.81, 0.01);
        time += dt;

        let x_scale = (p.x / 5.0) as usize;
        let y_scale = (p.y / 2.0) as usize;
        
        let mut line = String::from("");
        for _ in 0..x_scale { line.push(' '); }
        line.push('●');
        
        println!("T: {:.1}s | X: {:.1}m, Y: {:.1}m | {}", time, p.x, p.y, line);
        
        // In a real TUI we'd clear screen, but for a "walkthrough" log this is fine.
        // thread::sleep(Duration::from_millis(50));
    }
    
    println!("Impact! Final Distance: {:.2}m", p.x);
}
