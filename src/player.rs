use std::time::Duration;

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct Level(f64);

impl Level {
    pub fn get_down_duration(&self) -> Duration {
        Duration::from_secs_f64(0.5 - 0.05 * self.0)
    }
}

impl Default for Level {
    fn default() -> Self {
        Self(1.)
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Action {
    // Movement
    Left,
    Right,
    Down,
    RotateClock,
    RotateAnti,
}
#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    // This bundle must be added to your player entity
    // (or whatever else you wish to control)
    #[bundle]
    input_manager: InputManagerBundle<Action>,
}
impl PlayerBundle {
    fn default_input_map() -> InputMap<Action> {
        use Action::*;
        let mut input_map = InputMap::default();

        // Movement

        input_map.insert(KeyCode::Down, Down);
        input_map.insert(GamepadButtonType::DPadDown, Down);

        input_map.insert(KeyCode::Left, Left);
        input_map.insert(GamepadButtonType::DPadLeft, Left);

        input_map.insert(KeyCode::Right, Right);
        input_map.insert(GamepadButtonType::DPadRight, Right);

        input_map.insert(KeyCode::Up, RotateClock);
        input_map.insert(GamepadButtonType::Z, Right);

        input_map.insert(KeyCode::RShift, RotateAnti);
        input_map.insert(GamepadButtonType::C, Right);

        input_map
    }
}

pub fn spawn_player(commands: &mut Commands) {
    commands.spawn_bundle(PlayerBundle {
        player: Player,
        input_manager: InputManagerBundle {
            input_map: PlayerBundle::default_input_map(),
            action_state: ActionState::default(),
        },
    });
}
