use std::time::Duration;

use bevy::prelude::*;

use crate::MoveDownTimer;

pub struct LinesCompletedEvent(pub usize);

pub struct Level {
    pub level: f64,
    line_completed: usize,
}

impl Default for Level {
    fn default() -> Self {
        Self {
            level: 1.,
            line_completed: 0,
        }
    }
}

impl Level {
    pub fn get_down_duration(&self) -> Duration {
        Duration::from_secs_f64(0.5 - 0.05 * self.level)
    }
}

#[derive(Default)]
pub struct Score(usize);

pub fn increase_score_and_level(
    mut level: ResMut<Level>,
    mut score: ResMut<Score>,
    mut timer: ResMut<MoveDownTimer>,
    mut event_reader: EventReader<LinesCompletedEvent>,
) {
    for completed in event_reader.iter() {
        level.line_completed += completed.0;
        let old_level = level.level;
        level.level = (level.line_completed / 10) as f64 + 1.;
        if old_level != level.level {
            // adjust timer duration
            timer.timer.set_duration(level.get_down_duration());
        }
        score.0 += level.level as usize
            * completed.0
            * match completed.0 {
                4 => 10,
                _ => 7,
            };
        println!(
            "completed: {}\tlevel: {}\tscore: {}",
            level.line_completed, level.level, score.0
        );
    }
}
