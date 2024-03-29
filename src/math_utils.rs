use bevy::{
    prelude::*,
};

pub fn simple_bezier(a: Vec3, b: Vec3, c: Vec3, t: f32) -> Vec3{
    let ab = a.lerp(b, t);
    let bc = b.lerp(c, t);
    ab.lerp(bc, t)
}

// Find the points where the two circles intersect.
pub fn find_circle_circle_intersections(c0: Vec3, r0: f32, c1: Vec3, r1: f32) -> (Vec3, Vec3){
    // Find the distance between the centers.
    let dx= c0.x - c1.x;
    let dy = c0.y - c1.y;
    let dist = (dx * dx + dy * dy).sqrt();

    if (dist - (r0 + r1)).abs() < 0.00001
    {
        let intersection1 = c0.lerp(c1, r0 / (r0 + r1));
        let intersection2 = intersection1;
        return (intersection1, intersection2)
    }

    // See how many solutions there are.
    if dist > r0 + r1
    {
        // No solutions, the circles are too far apart.
        let intersection1 = Vec3::new(f32::NAN, f32::NAN, 0.0);
        let intersection2 = Vec3::new(f32::NAN, f32::NAN, 0.0);
        return (intersection1, intersection2)
    }
    else if dist < (r0 - r1).abs()
    {
        // No solutions, one circle contains the other.
        let intersection1 = Vec3::new(f32::NAN, f32::NAN, 0.0);
        let intersection2 = Vec3::new(f32::NAN, f32::NAN, 0.0);
        return (intersection1, intersection2)
    }
    else if (dist == 0.0) && (r0 == r1)
    {
        // No solutions, the circles coincide.
        let intersection1 = Vec3::new(f32::NAN, f32::NAN, 0.0);
        let intersection2 = Vec3::new(f32::NAN, f32::NAN, 0.0);
        return (intersection1, intersection2)
    }
    else
    {
        // Find a and h.
        let a = (r0 * r0 -
                    r1 * r1 + dist * dist) / (2.0 * dist);
        let h = (r0 * r0 - a * a).sqrt();

        // Find P2.
        let cx2 = c0.x + a * (c1.x - c0.x) / dist;
        let cy2 = c0.y + a * (c1.y - c0.y) / dist;

        // Get the points P3.
        let intersection1 = Vec3::new(
            cx2 + h * (c1.y - c0.y) / dist,
            cy2 - h * (c1.x - c0.x) / dist, 0.0);
        let intersection2 = Vec3::new(
            cx2 - h * (c1.y - c0.y) / dist,
            cy2 + h * (c1.x - c0.x) / dist, 0.0);

         return (intersection1, intersection2)
    }
}
