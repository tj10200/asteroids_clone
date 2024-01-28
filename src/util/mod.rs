use bevy::prelude::*;

pub fn angle_between(a: Vec2, b: Vec2) -> f32 {
    (a.dot(b) / (magnitude(a) * magnitude(b))).acos()
}

fn magnitude(a: Vec2) -> f32 {
    (a.x * a.x).sqrt() + (a.y * a.y).sqrt()
}

pub fn is_clockwise(from: Vec2, to: Vec2) -> bool {
    angle_between(from, to) <= 180f32
}
