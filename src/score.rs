use std::{fmt::Display, time::Duration};

use bevy::prelude::*;

use crate::{
    board::{BoardPosition, BOARD_HEIGHT, BOARD_WIDTH},
    MoveDownTimer,
};

pub struct LinesCompletedEvent(pub usize);

#[derive(Component)]
pub struct Level {
    pub level: f64,
}

#[derive(Component, Default)]
pub struct LineCompleted {
    line_completed: usize,
}

impl Default for Level {
    fn default() -> Self {
        Self { level: 1. }
    }
}

impl Level {
    pub fn get_down_duration(&self) -> Duration {
        Duration::from_secs_f64(0.5 - 0.05 * self.level)
    }
}

#[derive(Default, Component)]
pub struct Score(usize);

pub fn increase_score_and_level(
    mut level: Query<&mut Level>,
    mut lines: Query<&mut LineCompleted>,
    mut score: Query<&mut Score>,
    mut timer: ResMut<MoveDownTimer>,
    mut event_reader: EventReader<LinesCompletedEvent>,
) {
    for completed in event_reader.iter() {
        let mut score = score.single_mut();
        let mut level = level.single_mut();
        let mut lines = lines.single_mut();

        lines.line_completed += completed.0;
        let old_level = level.level;
        level.level = (lines.line_completed / 10) as f64 + 1.;
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
            lines.line_completed, level.level, score.0
        );
    }
}

pub fn setup_score(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("FiraCode-Regular.ttf");
    let text_style = TextStyle {
        font,
        font_size: 20.0,
        color: Color::ANTIQUE_WHITE,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Top,
        horizontal: HorizontalAlign::Left,
    };
    let initial_score = Score::default();
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(initial_score.to_string(), text_style.clone())
                .with_alignment(text_alignment),
            transform: BoardPosition::new(BOARD_WIDTH + 2, BOARD_HEIGHT - 4).to_real_position(),
            ..default()
        })
        .insert(initial_score);

    let initial_level = Level::default();
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(initial_level.to_string(), text_style.clone())
                .with_alignment(text_alignment),
            transform: BoardPosition::new(BOARD_WIDTH + 2, BOARD_HEIGHT - 2).to_real_position(),
            ..default()
        })
        .insert(initial_level);

    let initial_lines = LineCompleted::default();
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(initial_lines.to_string(), text_style.clone())
                .with_alignment(text_alignment),
            transform: BoardPosition::new(BOARD_WIDTH + 2, BOARD_HEIGHT).to_real_position(),
            ..default()
        })
        .insert(initial_lines);
}

pub fn dispayable_changed<T>(mut query: Query<(&T, &mut Text), Changed<T>>)
where
    T: Component + Display + 'static,
{
    query.for_each_mut(|(score, mut text)| text.sections[0].value = score.to_string());
}

impl Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SCORE\n{:05}", self.0)
    }
}
impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LEVEL\n{:02}", self.level)
    }
}
impl Display for LineCompleted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LINES\n{:03}", self.line_completed)
    }
}
