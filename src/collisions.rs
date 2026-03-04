pub fn elastic_collision(m1: f64, v1: f64, m2: f64, v2: f64) -> (f64, f64) {
    let v1_final = ((m1 - m2) * v1 + 2.0 * m2 * v2) / (m1 + m2);
    let v2_final = ((m2 - m1) * v2 + 2.0 * m1 * v1) / (m1 + m2);
    (v1_final, v2_final)
}
