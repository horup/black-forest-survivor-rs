use glam::Vec2;

/// Returns the intersection point of two line segments, if they intersect
pub fn line_intersect(
    line1: (Vec2, Vec2),
    line2: (Vec2, Vec2),
) -> Option<Vec2> {
    let (p1, p2) = line1;
    let (p3, p4) = line2;
    
    let x1 = p1.x;
    let y1 = p1.y;
    let x2 = p2.x;
    let y2 = p2.y;
    let x3 = p3.x;
    let y3 = p3.y;
    let x4 = p4.x;
    let y4 = p4.y;

    let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
    
    // Lines are parallel
    if denom.abs() < f32::EPSILON {
        return None;
    }

    let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denom;
    let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / denom;

    // Check if intersection is within both line segments
    if t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0 {
        let x = x1 + t * (x2 - x1);
        let y = y1 + t * (y2 - y1);
        Some(Vec2::new(x, y))
    } else {
        None
    }
}