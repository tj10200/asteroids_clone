use super::*;

#[derive(Resource)]
pub struct MeteorSpawnTimer {
    pub timer: Timer,
}

impl Default for MeteorSpawnTimer {
    fn default() -> MeteorSpawnTimer {
        MeteorSpawnTimer {
            timer: Timer::from_seconds(METEOR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
