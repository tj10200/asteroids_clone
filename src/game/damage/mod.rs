pub mod lib;

pub trait Damage {
    fn hit_points(&self) -> f32;
}
pub trait Damageable {
    fn damage(&mut self, entity: &impl Damage);

    fn health(&self) -> f32;

    fn is_dead(&self) -> bool {
        self.health() <= 0f32
    }
}
